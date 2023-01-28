use bevy::prelude::*;
use bevy_egui::egui;
use sugarfunge_api_types::{
    asset::{CreateClassInput, CreateClassOutput},
    primitives::{Account, ClassId, Seed},
};

use crate::{
    prelude::*,
    util::{request_handler, setup_in_out_channels},
};

use super::{ClassInputData, ClassOutputData};

#[derive(Debug)]
pub struct CreateClassRequest {
    pub input: CreateClassInput,
}

impl Request<CreateClassInput> for CreateClassRequest {
    fn endpoint(&self) -> &str {
        "asset/create_class"
    }

    fn input(&self) -> Option<CreateClassInput> {
        Some(CreateClassInput {
            seed: self.input.seed.clone(),
            class_id: self.input.class_id.clone(),
            metadata: self.input.metadata.clone(),
            owner: self.input.owner.clone(),
        })
    }
}

#[derive(Resource, Debug, Default, Clone)]
pub struct CreateClassInputData {
    pub seed: String,
    pub class_id: u64,
    pub metadata: String,
    pub owner: String,
    pub loading: bool,
}

pub fn create_class_ui(
    ui: &mut egui::Ui,
    class_input: &mut ResMut<ClassInputData>,
    created_tx: &Res<InputSender<CreateClassRequest>>,
    class_output: &Res<ClassOutputData>,
) {
    ui.label("Create Class");
    ui.separator();
    ui.label("Seed");
    ui.text_edit_singleline(&mut class_input.create_input.seed);
    ui.label("Class ID");
    ui.add(egui::DragValue::new::<u64>(&mut class_input.create_input.class_id).speed(0.1));
    ui.label("Metadata");
    ui.text_edit_multiline(&mut class_input.create_input.metadata);
    ui.label("Owner");
    ui.text_edit_singleline(&mut class_input.create_input.owner);
    if class_input.create_input.loading {
        ui.separator();
        ui.add(egui::Spinner::default());
    } else {
        if ui.button("Create").clicked() {
            created_tx
                .0
                .send(CreateClassRequest {
                    input: CreateClassInput {
                        seed: Seed::from(class_input.create_input.seed.clone()),
                        class_id: ClassId::from(class_input.create_input.class_id.clone()),
                        metadata: serde_json::from_str(&class_input.create_input.metadata).unwrap(),
                        owner: Account::from(class_input.create_input.owner.clone()),
                    },
                })
                .unwrap();

            class_input.create_input.loading = true;
        }
    }
    if let Some(output) = &class_output.create_output {
        ui.separator();
        ui.label("Who");
        ui.text_edit_singleline(&mut output.who.as_str());
        ui.label("Class ID");
        ui.text_edit_singleline(&mut u64::from(output.class_id).to_string());
    }
}

pub fn handle_create_response(
    mut class_output: ResMut<ClassOutputData>,
    mut class_input: ResMut<ClassInputData>,
    created_rx: Res<OutputReceiver<CreateClassOutput>>,
) {
    if let Ok(created_result) = created_rx.0.try_recv() {
        if let Some(created) = created_result {
            class_output.create_output = Some(created);
        }
        class_input.create_input.loading = false;
    }
}

pub struct CreateClassPlugin;

impl Plugin for CreateClassPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_in_out_channels::<CreateClassRequest, CreateClassOutput>)
            .add_system(request_handler::<CreateClassRequest, CreateClassInput, CreateClassOutput>)
            .add_system(handle_create_response);
    }
}
