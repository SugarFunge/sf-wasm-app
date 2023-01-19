use bevy::prelude::*;
use crossbeam::channel;
use serde::{Deserialize, Serialize};
use serde_json::json;

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
    let (output_sender, output_receiver) = channel::unbounded::<E>();
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
    let output_tx: channel::Sender<K> = output_tx.0.clone();
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
                    output_tx.send(value).unwrap();
                }
                Err(err) => {
                    error!("Request error: {:?}", err);
                }
            }
        });
    }
}
