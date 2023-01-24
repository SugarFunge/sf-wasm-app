use bevy::prelude::*;
use bevy_egui::egui;
use sugarfunge_api_types::{
    bundle::{BundleSchema, RegisterBundleInput, RegisterBundleOutput},
    primitives::{AssetId, Balance, ClassId, Seed},
};

use crate::{prelude::*, util::*};

use super::{BundleInputData, BundleOutputData};

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

#[derive(Resource, Debug, Default, Clone)]
pub struct RegisterBundleInputData {
    pub seed: String,
    pub class_id: u64,
    pub metadata: String,
    pub asset_id: u64,
    pub schema_class_ids: Vec<u64>,
    pub schema_asset_ids: Vec<Vec<u64>>,
    pub schema_amounts: Vec<Vec<u64>>,
    pub loading: bool,
}

pub fn register_bundle_ui(
    ui: &mut egui::Ui,
    bundle_input: &mut ResMut<BundleInputData>,
    registered_tx: &Res<InputSender<RegisterBundleRequest>>,
    bundle_output: &Res<BundleOutputData>,
) {
    ui.label("Register Bundle");
    ui.separator();
    ui.label("Seed");
    ui.text_edit_singleline(&mut bundle_input.register_input.seed);
    ui.label("Class ID");
    ui.add(egui::DragValue::new::<u64>(&mut bundle_input.register_input.class_id).speed(0.1));
    ui.label("Asset ID");
    ui.add(egui::DragValue::new::<u64>(&mut bundle_input.register_input.asset_id).speed(0.1));
    ui.label("Metadata");
    ui.text_edit_multiline(&mut bundle_input.register_input.metadata);
    ui.label("Schema Class IDs");
    vec_u64_input_ui(ui, &mut bundle_input.register_input.schema_class_ids);
    ui.label("Schema Asset IDs");
    vec_of_vec_u64_input_ui(
        ui,
        &mut bundle_input.register_input.schema_asset_ids,
        "Asset ID",
    );
    ui.label("Schema Amounts");
    ui.label("The Amounts are represented in 10^18 units.");
    vec_of_vec_u64_input_ui(
        ui,
        &mut bundle_input.register_input.schema_amounts,
        "Amount",
    );
    ui.separator();
    if ui.button("Register").clicked() {
        let class_ids: Vec<ClassId> =
            transform_vec_of_u64_to_class_id(bundle_input.register_input.schema_class_ids.clone());
        let asset_ids: Vec<Vec<AssetId>> = transform_doublevec_of_u64_to_asset_id(
            bundle_input.register_input.schema_asset_ids.clone(),
        );
        let amounts: Vec<Vec<Balance>> = transform_doublevec_of_u64_to_balance(
            bundle_input.register_input.schema_amounts.clone(),
        );
        registered_tx
            .send(RegisterBundleRequest {
                input: RegisterBundleInput {
                    seed: Seed::from(bundle_input.register_input.seed.clone()),
                    class_id: ClassId::from(bundle_input.register_input.class_id.clone()),
                    metadata: serde_json::from_str(&bundle_input.register_input.metadata.clone())
                        .unwrap(),
                    asset_id: AssetId::from(bundle_input.register_input.asset_id.clone()),
                    schema: BundleSchema {
                        class_ids,
                        asset_ids,
                        amounts,
                    },
                },
            })
            .unwrap();
        bundle_input.register_input.loading = true;
    }
    if let Some(output) = &bundle_output.register_output {
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

pub fn handle_register_response(
    mut bundle_output: ResMut<BundleOutputData>,
    mut bundle_input: ResMut<BundleInputData>,
    registered_rx: Res<OutputReceiver<RegisterBundleOutput>>,
) {
    if let Ok(registered_result) = registered_rx.0.try_recv() {
        if let Some(registered) = registered_result {
            bundle_output.register_output = Some(registered);
        }
        bundle_input.register_input.loading = false;
    }
}

pub struct RegisterBundlePlugin;

impl Plugin for RegisterBundlePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(
            setup_in_out_channels::<RegisterBundleRequest, RegisterBundleOutput>,
        )
        .add_system(
            request_handler::<RegisterBundleRequest, RegisterBundleInput, RegisterBundleOutput>,
        )
        .add_system(handle_register_response);
    }
}
