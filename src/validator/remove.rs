use bevy::prelude::*;
use bevy_egui::egui;
use crossbeam::channel;
use sugarfunge_api_types::{
    primitives::{Seed, ValidatorId},
    validator::{RemoveValidatorInput, RemoveValidatorOutput},
};

use crate::{prelude::*, util::request_handler};

use super::ValidatorUi;

#[derive(Debug)]
pub struct RemoveValidatorRequest {
    pub input: RemoveValidatorInput,
}

impl Request<RemoveValidatorInput> for RemoveValidatorRequest {
    fn endpoint(&self) -> &str {
        "validator/remove_validator"
    }

    fn input(&self) -> Option<RemoveValidatorInput> {
        Some(RemoveValidatorInput {
            seed: self.input.seed.clone(),
            validator_id: self.input.validator_id.clone(),
        })
    }
}

#[derive(Resource)]
pub struct RemoveValidatorChannel {
    pub input_tx: InputSender<RemoveValidatorRequest>,
    pub input_rx: InputReceiver<RemoveValidatorRequest>,
    pub output_tx: OutputSender<RemoveValidatorOutput>,
    pub output_rx: OutputReceiver<RemoveValidatorOutput>,
}

impl Default for RemoveValidatorChannel {
    fn default() -> Self {
        let (input_tx, input_rx) = channel::unbounded::<RemoveValidatorRequest>();
        let (output_tx, output_rx) = channel::unbounded::<Option<RemoveValidatorOutput>>();
        Self {
            input_tx: InputSender(input_tx),
            input_rx: InputReceiver(input_rx),
            output_tx: OutputSender(output_tx),
            output_rx: OutputReceiver(output_rx),
        }
    }
}

#[derive(Resource, Debug, Clone)]
pub struct RemoveValidatorInputData {
    pub seed: Seed,
    pub validator_id: ValidatorId,
    pub loading: bool,
}

impl Default for RemoveValidatorInputData {
    fn default() -> Self {
        Self {
            seed: Seed::from("".to_string()),
            validator_id: ValidatorId::from("".to_string()),
            loading: false,
        }
    }
}

pub fn remove_validator_ui(ui: &mut egui::Ui, validator: &mut ResMut<ValidatorUi>) {
    ui.label("Remove Validator");
    ui.separator();
    ui.label("Seed");
    ui.text_edit_singleline(&mut *validator.data.input.remove.seed);
    ui.label("Validator ID");
    ui.text_edit_singleline(&mut *validator.data.input.remove.validator_id);
    if validator.data.input.remove.loading {
        ui.separator();
        ui.add(egui::Spinner::default());
    } else {
        if ui.button("Remove").clicked() {
            validator
                .channels
                .remove
                .input_tx
                .0
                .send(RemoveValidatorRequest {
                    input: RemoveValidatorInput {
                        seed: validator.data.input.remove.seed.clone(),
                        validator_id: validator.data.input.remove.validator_id.clone(),
                    },
                })
                .unwrap();
            validator.data.input.remove.loading = true;
        }
    }
    if let Some(output) = &validator.data.output.remove {
        ui.separator();
        ui.label("Validator ID");
        ui.text_edit_singleline(&mut output.validator_id.as_str());
    }
}

pub fn handle_remove_response(
    mut validator: ResMut<ValidatorUi>,
    tokio_runtime: Res<TokioRuntime>,
) {
    if let Ok(removed_result) = validator.channels.remove.output_rx.0.try_recv() {
        if let Some(removed) = removed_result {
            validator.data.output.remove = Some(removed);
        }
        validator.data.input.remove.loading = false;
    }

    request_handler::<RemoveValidatorRequest, RemoveValidatorInput, RemoveValidatorOutput>(
        tokio_runtime.runtime.clone(),
        validator.channels.remove.input_rx.clone(),
        validator.channels.remove.output_tx.clone(),
    );
}

pub struct RemoveValidatorPlugin;

impl Plugin for RemoveValidatorPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(handle_remove_response);
    }
}
