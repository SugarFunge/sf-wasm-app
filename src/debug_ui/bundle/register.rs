use bevy::prelude::*;
use bevy_egui::egui;
use crossbeam::channel;
use sugarfunge_api_types::{
    bundle::{BundleSchema, RegisterBundleInput, RegisterBundleOutput},
    primitives::{AssetId, Balance, ClassId, Seed},
};

use crate::{prelude::*, util::*};

use super::BundleUi;

#[derive(Debug)]
pub struct RegisterBundleRequest {
    pub input: RegisterBundleInput,
}

impl Request<RegisterBundleInput> for RegisterBundleRequest {
    fn endpoint(&self) -> &str {
        "bundle/register"
    }

    fn input(&self) -> Option<RegisterBundleInput> {
        Some(RegisterBundleInput {
            seed: self.input.seed.clone(),
            class_id: self.input.class_id.clone(),
            metadata: self.input.metadata.clone(),
            asset_id: self.input.asset_id.clone(),
            schema: BundleSchema {
                class_ids: self.input.schema.class_ids.clone(),
                asset_ids: self.input.schema.asset_ids.clone(),
                amounts: self.input.schema.amounts.clone(),
            },
        })
    }
}

#[derive(Resource)]
pub struct RegisterBundleChannel {
    pub input_tx: InputSender<RegisterBundleRequest>,
    pub input_rx: InputReceiver<RegisterBundleRequest>,
    pub output_tx: OutputSender<RegisterBundleOutput>,
    pub output_rx: OutputReceiver<RegisterBundleOutput>,
}

impl Default for RegisterBundleChannel {
    fn default() -> Self {
        let (input_tx, input_rx) = channel::unbounded::<RegisterBundleRequest>();
        let (output_tx, output_rx) = channel::unbounded::<Option<RegisterBundleOutput>>();
        Self {
            input_tx: InputSender(input_tx),
            input_rx: InputReceiver(input_rx),
            output_tx: OutputSender(output_tx),
            output_rx: OutputReceiver(output_rx),
        }
    }
}

#[derive(Resource, Debug, Clone)]
pub struct RegisterBundleInputData {
    pub seed: Seed,
    pub class_id: ClassId,
    pub metadata: String,
    pub asset_id: AssetId,
    pub schema_class_ids: Vec<u64>,
    pub schema_asset_ids: Vec<Vec<u64>>,
    pub schema_amounts: Vec<Vec<u64>>,
    pub loading: bool,
}

impl Default for RegisterBundleInputData {
    fn default() -> Self {
        Self {
            seed: Seed::from("".to_string()),
            class_id: ClassId::from(0),
            metadata: "".to_string(),
            asset_id: AssetId::from(0),
            schema_class_ids: vec![],
            schema_asset_ids: vec![],
            schema_amounts: vec![],
            loading: false,
        }
    }
}

pub fn register_bundle_ui(ui: &mut egui::Ui, bundle: &mut ResMut<BundleUi>) {
    ui.label("Register Bundle");
    ui.separator();
    ui.label("Seed");
    ui.text_edit_singleline(&mut *bundle.data.input.register.seed);
    ui.label("Class ID");
    ui.add(egui::DragValue::new::<u64>(&mut *bundle.data.input.register.class_id).speed(0.1));
    ui.label("Asset ID");
    ui.add(egui::DragValue::new::<u64>(&mut *bundle.data.input.register.asset_id).speed(0.1));
    ui.label("Metadata");
    ui.text_edit_multiline(&mut bundle.data.input.register.metadata);
    ui.label("Schema Class IDs");
    vec_u64_input_ui(ui, &mut bundle.data.input.register.schema_class_ids);
    ui.label("Schema Asset IDs");
    vec_of_vec_u64_input_ui(
        ui,
        &mut bundle.data.input.register.schema_asset_ids,
        "Asset ID",
    );
    ui.label("Schema Amounts");
    vec_of_vec_u64_input_ui(ui, &mut bundle.data.input.register.schema_amounts, "Amount");
    ui.separator();
    if ui.button("Register").clicked() {
        let class_ids: Vec<ClassId> =
            transform_vec_of_u64_to_class_id(bundle.data.input.register.schema_class_ids.clone());
        let asset_ids: Vec<Vec<AssetId>> = transform_doublevec_of_u64_to_asset_id(
            bundle.data.input.register.schema_asset_ids.clone(),
        );
        let amounts: Vec<Vec<Balance>> = transform_doublevec_of_u64_to_balance(
            bundle.data.input.register.schema_amounts.clone(),
        );
        bundle
            .channels
            .register
            .input_tx
            .0
            .send(RegisterBundleRequest {
                input: RegisterBundleInput {
                    seed: bundle.data.input.register.seed.clone(),
                    class_id: bundle.data.input.register.class_id.clone(),
                    metadata: serde_json::from_str(&bundle.data.input.register.metadata.clone())
                        .unwrap(),
                    asset_id: bundle.data.input.register.asset_id.clone(),
                    schema: BundleSchema {
                        class_ids,
                        asset_ids,
                        amounts,
                    },
                },
            })
            .unwrap();
        bundle.data.input.register.loading = true;
    }
    if let Some(output) = &bundle.data.output.register {
        ui.separator();
        ui.label("Bundle ID");
        ui.text_edit_singleline(&mut output.bundle_id.as_str());
        ui.label("Who");
        ui.text_edit_singleline(&mut output.who.as_str());
        ui.label("Class ID");
        ui.text_edit_singleline(&mut format!("{:?}", output.class_id));
        ui.label("Asset ID");
        ui.text_edit_singleline(&mut format!("{:?}", output.asset_id));
    }
}

pub fn handle_register_response(mut bundle: ResMut<BundleUi>, tokio_runtime: Res<TokioRuntime>) {
    if let Ok(registered_result) = bundle.channels.register.output_rx.0.try_recv() {
        if let Some(registered) = registered_result {
            bundle.data.output.register = Some(registered);
        }
        bundle.data.input.register.loading = false;
    }

    request_handler::<RegisterBundleRequest, RegisterBundleInput, RegisterBundleOutput>(
        tokio_runtime.runtime.clone(),
        bundle.channels.register.input_rx.clone(),
        bundle.channels.register.output_tx.clone(),
    );
}

pub struct RegisterBundlePlugin;

impl Plugin for RegisterBundlePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(handle_register_response);
    }
}
