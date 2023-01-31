use bevy::prelude::*;
use bevy_egui::egui;
use crossbeam::channel;
use sugarfunge_api_types::{
    asset::{CreateClassInput, CreateClassOutput},
    primitives::{Account, ClassId, Seed},
};

use crate::{prelude::*, util::request_handler};

use super::ClassUi;

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

#[derive(Resource)]
pub struct CreateClassChannel {
    pub input_tx: InputSender<CreateClassRequest>,
    pub input_rx: InputReceiver<CreateClassRequest>,
    pub output_tx: OutputSender<CreateClassOutput>,
    pub output_rx: OutputReceiver<CreateClassOutput>,
}

impl Default for CreateClassChannel {
    fn default() -> Self {
        let (input_tx, input_rx) = channel::unbounded::<CreateClassRequest>();
        let (output_tx, output_rx) = channel::unbounded::<Option<CreateClassOutput>>();
        Self {
            input_tx: InputSender(input_tx),
            input_rx: InputReceiver(input_rx),
            output_tx: OutputSender(output_tx),
            output_rx: OutputReceiver(output_rx),
        }
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

pub fn create_class_ui(ui: &mut egui::Ui, class: &mut ResMut<ClassUi>) {
    ui.label("Create Class");
    ui.separator();
    ui.label("Seed");
    ui.text_edit_singleline(&mut class.data.input.create.seed);
    ui.label("Class ID");
    ui.add(egui::DragValue::new::<u64>(&mut class.data.input.create.class_id).speed(0.1));
    ui.label("Metadata");
    ui.text_edit_multiline(&mut class.data.input.create.metadata);
    ui.label("Owner");
    ui.text_edit_singleline(&mut class.data.input.create.owner);
    if class.data.input.create.loading {
        ui.separator();
        ui.add(egui::Spinner::default());
    } else {
        if ui.button("Create").clicked() {
            class
                .channels
                .create
                .input_tx
                .0
                .send(CreateClassRequest {
                    input: CreateClassInput {
                        seed: Seed::from(class.data.input.create.seed.clone()),
                        class_id: ClassId::from(class.data.input.create.class_id.clone()),
                        metadata: serde_json::from_str(&class.data.input.create.metadata).unwrap(),
                        owner: Account::from(class.data.input.create.owner.clone()),
                    },
                })
                .unwrap();

            class.data.input.create.loading = true;
        }
    }
    if let Some(output) = &class.data.output.create {
        ui.separator();
        ui.label("Who");
        ui.text_edit_singleline(&mut output.who.as_str());
        ui.label("Class ID");
        ui.text_edit_singleline(&mut u64::from(output.class_id).to_string());
    }
}

pub fn handle_create_response(mut class: ResMut<ClassUi>, tokio_runtime: Res<TokioRuntime>) {
    if let Ok(created_result) = class.channels.create.output_rx.0.try_recv() {
        if let Some(created) = created_result {
            class.data.output.create = Some(created);
        }
        class.data.input.create.loading = false;
    }

    request_handler::<CreateClassRequest, CreateClassInput, CreateClassOutput>(
        tokio_runtime.runtime.clone(),
        class.channels.create.input_rx.clone(),
        class.channels.create.output_tx.clone(),
    );
}

pub struct CreateClassPlugin;

impl Plugin for CreateClassPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(handle_create_response);
    }
}
