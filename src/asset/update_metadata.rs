use bevy::prelude::*;
use bevy_egui::egui;
use sugarfunge_api_types::{
    asset::{UpdateMetadataInput, UpdateMetadataOutput},
    primitives::{AssetId, ClassId, Seed},
};

use crate::{
    prelude::*,
    util::{request_handler, setup_in_out_channels},
};

use super::{AssetInputData, AssetOutputData};

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

pub fn update_asset_metadata_ui(
    ui: &mut egui::Ui,
    asset_input: &mut ResMut<AssetInputData>,
    updated_tx: &Res<InputSender<UpdateMetadataRequest>>,
    asset_output: &Res<AssetOutputData>,
) {
    ui.label("Update Asset Metadata");
    ui.separator();
    ui.label("Seed");
    ui.text_edit_singleline(&mut *asset_input.update_metadata_input.seed);
    ui.label("Class ID");
    ui.add(
        egui::DragValue::new::<u64>(&mut *asset_input.update_metadata_input.class_id).speed(0.1),
    );
    ui.label("Asset ID");
    ui.add(
        egui::DragValue::new::<u64>(&mut *asset_input.update_metadata_input.asset_id).speed(0.1),
    );
    ui.label("Metadata");
    ui.text_edit_multiline(&mut asset_input.update_metadata_input.metadata);
    if asset_input.update_metadata_input.loading {
        ui.separator();
        ui.add(egui::Spinner::default());
    } else {
        if ui.button("Update").clicked() {
            updated_tx
                .0
                .send(UpdateMetadataRequest {
                    input: UpdateMetadataInput {
                        seed: asset_input.update_metadata_input.seed.clone(),
                        class_id: asset_input.update_metadata_input.class_id,
                        metadata: serde_json::from_str(&asset_input.update_metadata_input.metadata)
                            .unwrap(),
                        asset_id: asset_input.update_metadata_input.asset_id,
                    },
                })
                .unwrap();
            asset_input.update_metadata_input.loading = true;
        }
    }
    if let Some(output) = &asset_output.update_metadata_output {
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
    mut asset_output: ResMut<AssetOutputData>,
    mut asset_input: ResMut<AssetInputData>,
    response_rx: Res<OutputReceiver<UpdateMetadataOutput>>,
) {
    if let Ok(response_result) = response_rx.0.try_recv() {
        if let Some(response) = response_result {
            asset_output.update_metadata_output = Some(response);
        }
        asset_input.update_metadata_input.loading = false;
    }
}

pub struct UpdateAssetMetadataPlugin;

impl Plugin for UpdateAssetMetadataPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(
            setup_in_out_channels::<UpdateMetadataRequest, UpdateMetadataOutput>,
        )
        .add_system(
            request_handler::<UpdateMetadataRequest, UpdateMetadataInput, UpdateMetadataOutput>,
        )
        .add_system(handle_update_metadata_response);
    }
}
