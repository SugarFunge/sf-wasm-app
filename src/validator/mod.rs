use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};
use sugarfunge_api_types::validator::*;

use crate::prelude::*;

pub mod add;
pub mod remove;

#[derive(Resource, Debug, Default, Eq, PartialEq)]
pub enum ValidatorActions {
    #[default]
    AddValidator,
    RemoveValidator,
}

#[derive(Resource, Debug, Default, Clone)]
pub struct ValidatorInputData {
    add_input: add::AddValidatorInputData,
    remove_input: remove::RemoveValidatorInputData,
}

#[derive(Resource, Default, Debug)]
pub struct ValidatorOutputData {
    add_output: Option<AddValidatorOutput>,
    remove_output: Option<RemoveValidatorOutput>,
}

pub fn validator_ui(
    mut ctx: ResMut<EguiContext>,
    mut validator_actions: ResMut<ValidatorActions>,
    mut validator_input: ResMut<ValidatorInputData>,
    validator_output: Res<ValidatorOutputData>,
    add_validator_tx: Res<InputSender<add::AddValidatorRequest>>,
    remove_validator_tx: Res<InputSender<remove::RemoveValidatorRequest>>,
) {
    egui::Window::new("Validator")
        .scroll2([false, true])
        .show(&mut ctx.ctx_mut(), |ui| {
            ui.horizontal(|ui| {
                ui.selectable_value(
                    &mut *validator_actions,
                    ValidatorActions::AddValidator,
                    "Add",
                );
                ui.selectable_value(
                    &mut *validator_actions,
                    ValidatorActions::RemoveValidator,
                    "Remove",
                );
            });
            ui.separator();
            match &*validator_actions {
                ValidatorActions::AddValidator => {
                    add::add_validator_ui(
                        ui,
                        &mut validator_input,
                        &add_validator_tx,
                        &validator_output,
                    );
                }
                ValidatorActions::RemoveValidator => {
                    remove::remove_validator_ui(
                        ui,
                        &mut validator_input,
                        &remove_validator_tx,
                        &validator_output,
                    );
                }
            }
        });
}

pub struct ValidatorPlugin;

impl Plugin for ValidatorPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ValidatorActions>()
            .init_resource::<ValidatorInputData>()
            .init_resource::<ValidatorOutputData>()
            .add_system(validator_ui)
            .add_plugin(add::AddValidatorPlugin)
            .add_plugin(remove::RemoveValidatorPlugin);
    }
}
