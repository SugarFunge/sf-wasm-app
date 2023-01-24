use bevy::prelude::*;
use bevy_inspector_egui::egui;
use crossbeam::channel;
use serde::{Deserialize, Serialize};
use serde_json::json;
use sugarfunge_api_types::primitives::{AssetId, Balance, ClassId};

use crate::prelude::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestError {
    pub message: serde_json::Value,
    pub description: String,
}

fn endpoint(cmd: &str) -> String {
    format!("http://127.0.0.1:4000/{}", cmd)
}

pub async fn req<'a, I, O>(cmd: &str, args: I) -> Result<O, RequestError>
where
    I: Serialize,
    O: for<'de> Deserialize<'de>,
{
    let sf_res = reqwest::Client::new()
        .post(endpoint(cmd))
        .json(&args)
        .send()
        .await;

    match sf_res {
        Ok(res) => {
            if let Err(err) = res.error_for_status_ref() {
                match res.json::<RequestError>().await {
                    Ok(err) => Err(err),
                    Err(_) => Err(RequestError {
                        message: json!(format!("{:#?}", err)),
                        description: "Reqwest json error.".into(),
                    }),
                }
            } else {
                match res.json().await {
                    Ok(res) => Ok(res),
                    Err(err) => Err(RequestError {
                        message: json!(format!("{:#?}", err)),
                        description: "Reqwest json error.".into(),
                    }),
                }
            }
        }
        Err(err) => Err(RequestError {
            message: json!(format!("{:#?}", err)),
            description: "Reqwest error.".into(),
        }),
    }
}

pub fn setup_in_out_channels<T: Send + 'static, E: Send + 'static>(mut commands: Commands) {
    let (input_sender, input_receiver) = channel::unbounded::<T>();
    commands.insert_resource(InputSender(input_sender));
    commands.insert_resource(InputReceiver(input_receiver));
    let (output_sender, output_receiver) = channel::unbounded::<Option<E>>();
    commands.insert_resource(OutputSender(output_sender));
    commands.insert_resource(OutputReceiver(output_receiver));
}

pub fn request_handler<
    T: Request<E>,
    E: Serialize + Send + Sync,
    K: Send + Sync + 'static + for<'de> Deserialize<'de>,
>(
    tokio_runtime: Res<TokioRuntime>,
    input_rx: Res<InputReceiver<T>>,
    output_tx: Res<OutputSender<K>>,
) {
    let rt = tokio_runtime.runtime.clone();
    let output_tx: channel::Sender<Option<K>> = output_tx.0.clone();
    if let Ok(request) = input_rx.0.try_recv() {
        rt.spawn(async move {
            let result: Result<K, RequestError>;
            if let Some(input) = request.input() {
                result = req(&request.endpoint(), input).await;
            } else {
                result = req(&request.endpoint(), ()).await;
            }
            match result {
                Ok(value) => {
                    output_tx.send(Some(value)).unwrap();
                }
                Err(err) => {
                    output_tx.send(None).unwrap();
                    error!("Request error: {:?}", err);
                }
            }
        });
    }
}

pub fn vec_u64_input_ui(ui: &mut egui::Ui, input: &mut Vec<u64>) {
    if ui.button("Add").clicked() {
        input.push(u64::default());
    }
    let input_clone = input.clone();
    let mut remove_index: Option<usize> = None;
    for (i, _) in input_clone.iter().enumerate() {
        ui.horizontal(|ui| {
            ui.add(egui::DragValue::new::<u64>(&mut input[i]).speed(0.1));
            if ui.button("Remove").clicked() {
                remove_index = Some(i);
            }
        });
    }
    if let Some(index) = remove_index {
        input.remove(index);
    }
}

pub fn vec_of_vec_u64_input_ui(ui: &mut egui::Ui, input: &mut Vec<Vec<u64>>, label: &str) {
    if ui.button(format!("Add Vec of {}", label)).clicked() {
        input.push(vec![]);
    }
    let input_clone = input.clone();
    let mut remove_vec_index: Option<usize> = None;
    for (i, _) in input_clone.iter().enumerate() {
        ui.label(format!("Vec: {}: {}", label, i));
        vec_u64_input_ui(ui, &mut input[i]);
        if ui.button("Remove").clicked() {
            remove_vec_index = Some(i);
        }
        ui.separator();
    }
    if let Some(index) = remove_vec_index {
        input.remove(index);
    }
}

pub fn transform_vec_of_u64_to_class_id(input: Vec<u64>) -> Vec<ClassId> {
    input
        .iter()
        .map(|class_id| ClassId::from(*class_id))
        .collect()
}

pub fn transform_doublevec_of_u64_to_asset_id(input: Vec<Vec<u64>>) -> Vec<Vec<AssetId>> {
    input
        .iter()
        .map(|asset_ids| {
            asset_ids
                .iter()
                .map(|asset_id| AssetId::from(*asset_id))
                .collect()
        })
        .collect()
}

pub fn transform_doublevec_of_u64_to_balance(input: Vec<Vec<u64>>) -> Vec<Vec<Balance>> {
    input
        .iter()
        .map(|amounts| {
            amounts
                .iter()
                .map(|amount| Balance::from(u128::from(*amount) * (u128::pow(10, 18))))
                .collect()
        })
        .collect()
}
