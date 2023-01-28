use bevy::prelude::*;
use bevy_egui::egui;
use sugarfunge_api_types::{
    primitives::{Seed, ValidatorId},
    validator::{AddValidatorInput, AddValidatorOutput},
};

use crate::{prelude::*, util::*};

use super::{ValidatorInputData, ValidatorOutputData};

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

pub fn add_validator_ui(
    ui: &mut egui::Ui,
    validator_input: &mut ResMut<ValidatorInputData>,
    add_tx: &Res<InputSender<AddValidatorRequest>>,
    validator_output: &Res<ValidatorOutputData>,
) {
    ui.label("Add Validator");
    ui.separator();
    ui.label("Seed");
    ui.text_edit_singleline(&mut *validator_input.add_input.seed);
    ui.label("Validator ID");
    ui.text_edit_singleline(&mut *validator_input.add_input.validator_id);
    if validator_input.add_input.loading {
        ui.separator();
        ui.add(egui::Spinner::default());
    } else {
        if ui.button("Add").clicked() {
            add_tx
                .0
                .send(AddValidatorRequest {
                    input: AddValidatorInput {
                        seed: validator_input.add_input.seed.clone(),
                        validator_id: validator_input.add_input.validator_id.clone(),
                    },
                })
                .unwrap();
            validator_input.add_input.loading = true;
        }
    }
    if let Some(output) = &validator_output.add_output {
        ui.separator();
        ui.label("Validator ID");
        ui.text_edit_singleline(&mut output.validator_id.as_str());
    }
}

pub fn handle_add_response(
    mut validator_output: ResMut<ValidatorOutputData>,
    mut validator_input: ResMut<ValidatorInputData>,
    added_rx: Res<OutputReceiver<AddValidatorOutput>>,
) {
    if let Ok(added_result) = added_rx.0.try_recv() {
        if let Some(added) = added_result {
            validator_output.add_output = Some(added);
        }
        validator_input.add_input.loading = false;
    }
}

pub struct AddValidatorPlugin;

impl Plugin for AddValidatorPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_in_out_channels::<AddValidatorRequest, AddValidatorOutput>)
            .add_system(
                request_handler::<AddValidatorRequest, AddValidatorInput, AddValidatorOutput>,
            )
            .add_system(handle_add_response);
    }
}
