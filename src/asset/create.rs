use bevy::prelude::*;
use bevy_egui::egui;
use crossbeam::channel;
use sugarfunge_api_types::{
    asset::{CreateInput, CreateOutput},
    primitives::{AssetId, ClassId, Seed},
};

use crate::{prelude::*, util::request_handler};

use super::AssetUi;

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

#[derive(Resource)]
pub struct CreateAssetChannel {
    pub input_tx: InputSender<CreateAssetRequest>,
    pub input_rx: InputReceiver<CreateAssetRequest>,
    pub output_tx: OutputSender<CreateOutput>,
    pub output_rx: OutputReceiver<CreateOutput>,
}

impl Default for CreateAssetChannel {
    fn default() -> Self {
        let (input_tx, input_rx) = channel::unbounded::<CreateAssetRequest>();
        let (output_tx, output_rx) = channel::unbounded::<Option<CreateOutput>>();
        Self {
            input_tx: InputSender(input_tx),
            input_rx: InputReceiver(input_rx),
            output_tx: OutputSender(output_tx),
            output_rx: OutputReceiver(output_rx),
        }
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

pub fn create_asset_ui(ui: &mut egui::Ui, asset: &mut ResMut<AssetUi>) {
    ui.label("Create Asset");
    ui.separator();
    ui.label("Seed");
    ui.text_edit_singleline(&mut *asset.data.input.create.seed);
    ui.label("Class ID");
    ui.add(egui::DragValue::new::<u64>(&mut *asset.data.input.create.class_id).speed(0.1));
    ui.label("Metadata");
    ui.text_edit_multiline(&mut asset.data.input.create.metadata);
    ui.label("Asset ID");
    ui.add(egui::DragValue::new::<u64>(&mut *asset.data.input.create.asset_id).speed(0.1));
    if asset.data.input.create.loading {
        ui.separator();
        ui.add(egui::Spinner::default());
    } else {
        if ui.button("Create").clicked() {
            asset
                .channels
                .create
                .input_tx
                .0
                .send(CreateAssetRequest {
                    input: CreateInput {
                        seed: asset.data.input.create.seed.clone(),
                        class_id: asset.data.input.create.class_id,
                        metadata: serde_json::from_str(&asset.data.input.create.metadata).unwrap(),
                        asset_id: asset.data.input.create.asset_id.clone(),
                    },
                })
                .unwrap();
            asset.data.input.create.loading = true;
        }
    }
    if let Some(output) = &asset.data.output.create {
        ui.separator();
        ui.label("Class ID");
        ui.text_edit_singleline(&mut u64::from(output.class_id).to_string());
        ui.label("Asset ID");
        ui.text_edit_singleline(&mut u64::from(output.asset_id).to_string());
        ui.label("Who");
        ui.text_edit_singleline(&mut output.who.as_str());
    }
}

pub fn handle_create_response(mut asset: ResMut<AssetUi>, tokio_runtime: Res<TokioRuntime>) {
    if let Ok(created_result) = asset.channels.create.output_rx.0.try_recv() {
        if let Some(created) = created_result {
            asset.data.output.create = Some(created);
        }
        asset.data.input.create.loading = false;
    }

    request_handler::<CreateAssetRequest, CreateInput, CreateOutput>(
        tokio_runtime.runtime.clone(),
        asset.channels.create.input_rx.clone(),
        asset.channels.create.output_tx.clone(),
    );
}

pub struct CreateAssetPlugin;

impl Plugin for CreateAssetPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(handle_create_response);
    }
}
