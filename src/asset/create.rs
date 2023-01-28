use bevy::prelude::*;
use bevy_egui::egui;
use sugarfunge_api_types::{
    asset::{CreateInput, CreateOutput},
    primitives::{AssetId, ClassId, Seed},
};

use crate::{
    prelude::*,
    util::{request_handler, setup_in_out_channels},
};

use super::{AssetInputData, AssetOutputData};

#[derive(Debug)]
pub struct CreateAssetRequest {
    pub input: CreateInput,
}

impl Request<CreateInput> for CreateAssetRequest {
    fn endpoint(&self) -> &str {
        "asset/create"
    }

    fn input(&self) -> Option<CreateInput> {
        Some(CreateInput {
            seed: self.input.seed.clone(),
            class_id: self.input.class_id.clone(),
            metadata: self.input.metadata.clone(),
            asset_id: self.input.asset_id.clone(),
        })
    }
}

#[derive(Resource, Debug, Clone)]
pub struct CreateAssetInputData {
    pub seed: Seed,
    pub class_id: ClassId,
    pub metadata: String,
    pub asset_id: AssetId,
    pub loading: bool,
}

impl Default for CreateAssetInputData {
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

pub fn create_asset_ui(
    ui: &mut egui::Ui,
    asset_input: &mut ResMut<AssetInputData>,
    create_tx: &Res<InputSender<CreateAssetRequest>>,
    asset_output: &Res<AssetOutputData>,
) {
    ui.label("Create Asset");
    ui.separator();
    ui.label("Seed");
    ui.text_edit_singleline(&mut *asset_input.create_input.seed);
    ui.label("Class ID");
    ui.add(egui::DragValue::new::<u64>(&mut *asset_input.create_input.class_id).speed(0.1));
    ui.label("Metadata");
    ui.text_edit_multiline(&mut asset_input.create_input.metadata);
    ui.label("Asset ID");
    ui.add(egui::DragValue::new::<u64>(&mut *asset_input.create_input.asset_id).speed(0.1));
    if asset_input.create_input.loading {
        ui.separator();
        ui.add(egui::Spinner::default());
    } else {
        if ui.button("Create").clicked() {
            create_tx
                .0
                .send(CreateAssetRequest {
                    input: CreateInput {
                        seed: asset_input.create_input.seed.clone(),
                        class_id: asset_input.create_input.class_id,
                        metadata: serde_json::from_str(&asset_input.create_input.metadata).unwrap(),
                        asset_id: asset_input.create_input.asset_id.clone(),
                    },
                })
                .unwrap();
            asset_input.create_input.loading = true;
        }
    }
    if let Some(output) = &asset_output.create_output {
        ui.separator();
        ui.label("Class ID");
        ui.text_edit_singleline(&mut u64::from(output.class_id).to_string());
        ui.label("Asset ID");
        ui.text_edit_singleline(&mut u64::from(output.asset_id).to_string());
        ui.label("Who");
        ui.text_edit_singleline(&mut output.who.as_str());
    }
}

pub fn handle_create_response(
    mut asset_output: ResMut<AssetOutputData>,
    mut asset_input: ResMut<AssetInputData>,
    created_rx: Res<OutputReceiver<CreateOutput>>,
) {
    if let Ok(created_result) = created_rx.0.try_recv() {
        if let Some(created) = created_result {
            asset_output.create_output = Some(created);
        }
        asset_input.create_input.loading = false;
    }
}

pub struct CreateAssetPlugin;

impl Plugin for CreateAssetPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_in_out_channels::<CreateAssetRequest, CreateOutput>)
            .add_system(request_handler::<CreateAssetRequest, CreateInput, CreateOutput>)
            .add_system(handle_create_response);
    }
}
