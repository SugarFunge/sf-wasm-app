use bevy::prelude::*;
use bevy_egui::egui;
use sugarfunge_api_types::{
    bag::{RegisterInput, RegisterOutput},
    primitives::{ClassId, Seed},
};

use crate::{
    prelude::*,
    util::{request_handler, setup_in_out_channels},
};

use super::{BagInputData, BagOutputData};

#[derive(Debug)]
pub struct RegisterBagRequest {
    pub input: RegisterInput,
}

impl Request<RegisterInput> for RegisterBagRequest {
    fn endpoint(&self) -> &str {
        "bag/register"
    }

    fn input(&self) -> Option<RegisterInput> {
        Some(RegisterInput {
            seed: self.input.seed.clone(),
            class_id: self.input.class_id.clone(),
            metadata: self.input.metadata.clone(),
        })
    }
}

#[derive(Resource, Debug, Default, Clone)]
pub struct RegisterBagInputData {
    pub seed: String,
    pub class_id: u64,
    pub metadata: String,
    pub loading: bool,
}

pub fn register_bag_ui(
    ui: &mut egui::Ui,
    bag_input: &mut ResMut<BagInputData>,
    registered_tx: &Res<InputSender<RegisterBagRequest>>,
    bag_output: &Res<BagOutputData>,
) {
    ui.label("Register Bag");
    ui.separator();
    ui.label("Seed");
    ui.text_edit_singleline(&mut bag_input.register_input.seed);
    ui.label("Class ID");
    ui.add(egui::DragValue::new::<u64>(&mut bag_input.register_input.class_id).speed(0.1));
    ui.label("Metadata");
    ui.text_edit_multiline(&mut bag_input.register_input.metadata);
    if bag_input.register_input.loading {
        ui.separator();
        ui.add(egui::Spinner::default());
    } else {
        if ui.button("Register").clicked() {
            registered_tx
                .send(RegisterBagRequest {
                    input: RegisterInput {
                        seed: Seed::from(bag_input.register_input.seed.clone()),
                        class_id: ClassId::from(bag_input.register_input.class_id),
                        metadata: serde_json::from_str(&bag_input.register_input.metadata).unwrap(),
                    },
                })
                .unwrap();
            bag_input.register_input.loading = true;
        }
    }
    if let Some(output) = &bag_output.register_output {
        ui.separator();
        ui.label("Who");
        ui.text_edit_singleline(&mut output.who.as_str());
        ui.label("Class ID");
        ui.text_edit_singleline(&mut u64::from(output.class_id).to_string());
    }
}

pub fn handle_register_response(
    mut bag_output: ResMut<BagOutputData>,
    mut bag_input: ResMut<BagInputData>,
    registered_rx: Res<OutputReceiver<RegisterOutput>>,
) {
    if let Ok(registered_result) = registered_rx.0.try_recv() {
        if let Some(registered) = registered_result {
            bag_output.register_output = Some(registered);
        }
        bag_input.register_input.loading = false;
    }
}

pub struct RegisterBagPlugin;

impl Plugin for RegisterBagPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_in_out_channels::<RegisterBagRequest, RegisterOutput>)
            .add_system(request_handler::<RegisterBagRequest, RegisterInput, RegisterOutput>)
            .add_system(handle_register_response);
    }
}
