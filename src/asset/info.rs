use bevy::prelude::*;
use bevy_egui::egui;
use crossbeam::channel;
use sugarfunge_api_types::{
    asset::{AssetInfoInput, AssetInfoOutput},
    primitives::{AssetId, ClassId},
};

use crate::{prelude::*, util::request_handler};

use super::AssetUi;

#[derive(Debug)]
pub struct AssetInfoRequest {
    pub input: AssetInfoInput,
}

impl Request<AssetInfoInput> for AssetInfoRequest {
    fn endpoint(&self) -> &str {
        "asset/info"
    }

    fn input(&self) -> Option<AssetInfoInput> {
        Some(AssetInfoInput {
            asset_id: self.input.asset_id.clone(),
            class_id: self.input.class_id.clone(),
        })
    }
}

#[derive(Resource)]
pub struct AssetInfoChannel {
    pub input_tx: InputSender<AssetInfoRequest>,
    pub input_rx: InputReceiver<AssetInfoRequest>,
    pub output_tx: OutputSender<AssetInfoOutput>,
    pub output_rx: OutputReceiver<AssetInfoOutput>,
}

impl Default for AssetInfoChannel {
    fn default() -> Self {
        let (input_tx, input_rx) = channel::unbounded::<AssetInfoRequest>();
        let (output_tx, output_rx) = channel::unbounded::<Option<AssetInfoOutput>>();
        Self {
            input_tx: InputSender(input_tx),
            input_rx: InputReceiver(input_rx),
            output_tx: OutputSender(output_tx),
            output_rx: OutputReceiver(output_rx),
        }
    }
}

#[derive(Resource, Debug, Clone)]
pub struct AssetInfoInputData {
    pub asset_id: AssetId,
    pub class_id: ClassId,
    pub loading: bool,
}

impl Default for AssetInfoInputData {
    fn default() -> Self {
        Self {
            asset_id: AssetId::from(0),
            class_id: ClassId::from(0),
            loading: false,
        }
    }
}

pub fn asset_info_ui(ui: &mut egui::Ui, asset: &mut ResMut<AssetUi>) {
    ui.label("Asset Info");
    ui.separator();
    ui.label("Class ID");
    ui.add(egui::DragValue::new::<u64>(&mut *asset.data.input.info.class_id).speed(0.1));
    ui.label("Asset ID");
    ui.add(egui::DragValue::new::<u64>(&mut *asset.data.input.info.asset_id).speed(0.1));
    if asset.data.input.info.loading {
        ui.separator();
        ui.add(egui::Spinner::default());
    } else {
        if ui.button("Get Info").clicked() {
            asset
                .channels
                .info
                .input_tx
                .0
                .send(AssetInfoRequest {
                    input: AssetInfoInput {
                        asset_id: asset.data.input.info.asset_id,
                        class_id: asset.data.input.info.class_id,
                    },
                })
                .unwrap();
            asset.data.input.info.loading = true;
        }
    }
    if let Some(output) = &asset.data.output.info {
        ui.separator();
        if let Some(info) = &output.info {
            ui.label("Class ID");
            ui.text_edit_singleline(&mut format!("{:?}", info.class_id));
            ui.label("Asset ID");
            ui.text_edit_singleline(&mut format!("{:?}", info.asset_id));
            ui.label("Metadata");
            ui.text_edit_singleline(&mut info.metadata.to_string());
        } else {
            ui.label("No asset info found");
        }
    }
}

pub fn handle_info_response(mut asset: ResMut<AssetUi>, tokio_runtime: Res<TokioRuntime>) {
    if let Ok(info_result) = asset.channels.info.output_rx.0.try_recv() {
        if let Some(info) = info_result {
            asset.data.output.info = Some(info);
        }
        asset.data.input.info.loading = false;
    }

    request_handler::<AssetInfoRequest, AssetInfoInput, AssetInfoOutput>(
        tokio_runtime.runtime.clone(),
        asset.channels.info.input_rx.clone(),
        asset.channels.info.output_tx.clone(),
    );
}

pub struct AssetInfoPlugin;

impl Plugin for AssetInfoPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(handle_info_response);
    }
}
