use bevy::prelude::*;
use bevy_egui::egui;
use crossbeam::channel;
use sugarfunge_api_types::{
    asset::{UpdateMetadataInput, UpdateMetadataOutput},
    primitives::{AssetId, ClassId, Seed},
};

use crate::{prelude::*, util::request_handler};

use super::AssetUi;

#[derive(Debug)]
pub struct UpdateMetadataRequest {
    pub input: UpdateMetadataInput,
}

impl Request<UpdateMetadataInput> for UpdateMetadataRequest {
    fn endpoint(&self) -> &str {
        "asset/update_metadata"
    }

    fn input(&self) -> Option<UpdateMetadataInput> {
        Some(UpdateMetadataInput {
            seed: self.input.seed.clone(),
            class_id: self.input.class_id.clone(),
            metadata: self.input.metadata.clone(),
            asset_id: self.input.asset_id.clone(),
        })
    }
}

#[derive(Resource)]
pub struct UpdateMetadataChannel {
    pub input_tx: InputSender<UpdateMetadataRequest>,
    pub input_rx: InputReceiver<UpdateMetadataRequest>,
    pub output_tx: OutputSender<UpdateMetadataOutput>,
    pub output_rx: OutputReceiver<UpdateMetadataOutput>,
}

impl Default for UpdateMetadataChannel {
    fn default() -> Self {
        let (input_tx, input_rx) = channel::unbounded::<UpdateMetadataRequest>();
        let (output_tx, output_rx) = channel::unbounded::<Option<UpdateMetadataOutput>>();
        Self {
            input_tx: InputSender(input_tx),
            input_rx: InputReceiver(input_rx),
            output_tx: OutputSender(output_tx),
            output_rx: OutputReceiver(output_rx),
        }
    }
}

#[derive(Resource, Debug, Clone)]
pub struct UpdateAssetMetadataInputData {
    pub seed: Seed,
    pub class_id: ClassId,
    pub metadata: String,
    pub asset_id: AssetId,
    pub loading: bool,
}

impl Default for UpdateAssetMetadataInputData {
    fn default() -> Self {
        Self {
            seed: Seed::from("".to_string()),
            class_id: ClassId::from(0),
            metadata: "".to_string(),
            asset_id: AssetId::from(0),
            loading: false,
        }
    }
}

pub fn update_asset_metadata_ui(ui: &mut egui::Ui, asset: &mut ResMut<AssetUi>) {
    ui.label("Update Asset Metadata");
    ui.separator();
    ui.label("Seed");
    ui.text_edit_singleline(&mut *asset.data.input.update_metadata.seed);
    ui.label("Class ID");
    ui.add(egui::DragValue::new::<u64>(&mut *asset.data.input.update_metadata.class_id).speed(0.1));
    ui.label("Asset ID");
    ui.add(egui::DragValue::new::<u64>(&mut *asset.data.input.update_metadata.asset_id).speed(0.1));
    ui.label("Metadata");
    ui.text_edit_multiline(&mut asset.data.input.update_metadata.metadata);
    if asset.data.input.update_metadata.loading {
        ui.separator();
        ui.add(egui::Spinner::default());
    } else {
        if ui.button("Update").clicked() {
            asset
                .channels
                .update_metadata
                .input_tx
                .0
                .send(UpdateMetadataRequest {
                    input: UpdateMetadataInput {
                        seed: asset.data.input.update_metadata.seed.clone(),
                        class_id: asset.data.input.update_metadata.class_id,
                        metadata: serde_json::from_str(&asset.data.input.update_metadata.metadata)
                            .unwrap(),
                        asset_id: asset.data.input.update_metadata.asset_id,
                    },
                })
                .unwrap();
            asset.data.input.update_metadata.loading = true;
        }
    }
    if let Some(output) = &asset.data.output.update_metadata {
        ui.separator();
        ui.label("Asset ID");
        ui.text_edit_singleline(&mut u64::from(output.asset_id).to_string());
        ui.label("Class ID");
        ui.text_edit_singleline(&mut u64::from(output.class_id).to_string());
        ui.label("Who");
        ui.text_edit_singleline(&mut output.who.as_str());
        ui.label("Metadata");
        ui.text_edit_multiline(&mut output.metadata.to_string());
    }
}

pub fn handle_update_metadata_response(
    mut asset: ResMut<AssetUi>,
    tokio_runtime: Res<TokioRuntime>,
) {
    if let Ok(response_result) = asset.channels.update_metadata.output_rx.0.try_recv() {
        if let Some(response) = response_result {
            asset.data.output.update_metadata = Some(response);
        }
        asset.data.input.update_metadata.loading = false;
    }

    request_handler::<UpdateMetadataRequest, UpdateMetadataInput, UpdateMetadataOutput>(
        tokio_runtime.runtime.clone(),
        asset.channels.update_metadata.input_rx.clone(),
        asset.channels.update_metadata.output_tx.clone(),
    );
}

pub struct UpdateAssetMetadataPlugin;

impl Plugin for UpdateAssetMetadataPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(handle_update_metadata_response);
    }
}
