use bevy::prelude::*;
use bevy_egui::egui;
use crossbeam::channel;
use sugarfunge_api_types::{
    primitives::{Seed, ValidatorId},
    validator::{AddValidatorInput, AddValidatorOutput},
};

use crate::{prelude::*, util::request_handler};

use super::ValidatorUi;

#[derive(Debug)]
pub struct AddValidatorRequest {
    pub input: AddValidatorInput,
}

impl Request<AddValidatorInput> for AddValidatorRequest {
    fn endpoint(&self) -> &str {
        "validator/add_validator"
    }

    fn input(&self) -> Option<AddValidatorInput> {
        Some(AddValidatorInput {
            seed: self.input.seed.clone(),
            validator_id: self.input.validator_id.clone(),
        })
    }
}

#[derive(Resource)]
pub struct AddValidatorChannel {
    pub input_tx: InputSender<AddValidatorRequest>,
    pub input_rx: InputReceiver<AddValidatorRequest>,
    pub output_tx: OutputSender<AddValidatorOutput>,
    pub output_rx: OutputReceiver<AddValidatorOutput>,
}

impl Default for AddValidatorChannel {
    fn default() -> Self {
        let (input_tx, input_rx) = channel::unbounded::<AddValidatorRequest>();
        let (output_tx, output_rx) = channel::unbounded::<Option<AddValidatorOutput>>();
        Self {
            input_tx: InputSender(input_tx),
            input_rx: InputReceiver(input_rx),
            output_tx: OutputSender(output_tx),
            output_rx: OutputReceiver(output_rx),
        }
    }
}

#[derive(Resource, Debug, Clone)]
pub struct AddValidatorInputData {
    pub seed: Seed,
    pub validator_id: ValidatorId,
    pub loading: bool,
}

impl Default for AddValidatorInputData {
    fn default() -> Self {
        Self {
            seed: Seed::from("".to_string()),
            validator_id: ValidatorId::from("".to_string()),
            loading: false,
        }
    }
}

pub fn add_validator_ui(ui: &mut egui::Ui, validator: &mut ResMut<ValidatorUi>) {
    ui.label("Add Validator");
    ui.separator();
    ui.label("Seed");
    ui.text_edit_singleline(&mut *validator.data.input.add.seed);
    ui.label("Validator ID");
    ui.text_edit_singleline(&mut *validator.data.input.add.validator_id);
    if validator.data.input.add.loading {
        ui.separator();
        ui.add(egui::Spinner::default());
    } else {
        if ui.button("Add").clicked() {
            validator
                .channels
                .add
                .input_tx
                .0
                .send(AddValidatorRequest {
                    input: AddValidatorInput {
                        seed: validator.data.input.add.seed.clone(),
                        validator_id: validator.data.input.add.validator_id.clone(),
                    },
                })
                .unwrap();
            validator.data.input.add.loading = true;
        }
    }
    if let Some(output) = &validator.data.output.add {
        ui.separator();
        ui.label("Validator ID");
        ui.text_edit_singleline(&mut output.validator_id.as_str());
    }
}

pub fn handle_add_response(mut validator: ResMut<ValidatorUi>, tokio_runtime: Res<TokioRuntime>) {
    if let Ok(added_result) = validator.channels.add.output_rx.0.try_recv() {
        if let Some(added) = added_result {
            validator.data.output.add = Some(added);
        }
        validator.data.input.add.loading = false;
    }

    request_handler::<AddValidatorRequest, AddValidatorInput, AddValidatorOutput>(
        tokio_runtime.runtime.clone(),
        validator.channels.add.input_rx.clone(),
        validator.channels.add.output_tx.clone(),
    );
}

pub struct AddValidatorPlugin;

impl Plugin for AddValidatorPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(handle_add_response);
    }
}
