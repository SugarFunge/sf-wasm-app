use bevy::prelude::*;
use bevy_egui::egui;
use sugarfunge_api_types::{
    primitives::{Seed, ValidatorId},
    validator::{RemoveValidatorInput, RemoveValidatorOutput},
};

use crate::{prelude::*, util::*};

use super::{ValidatorInputData, ValidatorOutputData};

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

pub fn remove_validator_ui(
    ui: &mut egui::Ui,
    validator_input: &mut ResMut<ValidatorInputData>,
    remove_tx: &Res<InputSender<RemoveValidatorRequest>>,
    validator_output: &Res<ValidatorOutputData>,
) {
    ui.label("Remove Validator");
    ui.separator();
    ui.label("Seed");
    ui.text_edit_singleline(&mut *validator_input.remove_input.seed);
    ui.label("Validator ID");
    ui.text_edit_singleline(&mut *validator_input.remove_input.validator_id);
    if validator_input.remove_input.loading {
        ui.separator();
        ui.add(egui::Spinner::default());
    } else {
        if ui.button("Remove").clicked() {
            remove_tx
                .0
                .send(RemoveValidatorRequest {
                    input: RemoveValidatorInput {
                        seed: validator_input.remove_input.seed.clone(),
                        validator_id: validator_input.remove_input.validator_id.clone(),
                    },
                })
                .unwrap();
            validator_input.remove_input.loading = true;
        }
    }
    if let Some(output) = &validator_output.remove_output {
        ui.separator();
        ui.label("Validator ID");
        ui.text_edit_singleline(&mut output.validator_id.as_str());
    }
}

pub fn handle_remove_response(
    mut validator_output: ResMut<ValidatorOutputData>,
    mut validator_input: ResMut<ValidatorInputData>,
    removed_rx: Res<OutputReceiver<RemoveValidatorOutput>>,
) {
    if let Ok(removed_result) = removed_rx.0.try_recv() {
        if let Some(removed) = removed_result {
            validator_output.remove_output = Some(removed);
        }
        validator_input.remove_input.loading = false;
    }
}

pub struct RemoveValidatorPlugin;

impl Plugin for RemoveValidatorPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_in_out_channels::<RemoveValidatorRequest, RemoveValidatorOutput>)
            .add_system(
                request_handler::<RemoveValidatorRequest, RemoveValidatorInput, RemoveValidatorOutput>,
            )
            .add_system(handle_remove_response);
    }
}
