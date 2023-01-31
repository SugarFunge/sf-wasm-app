use bevy::prelude::*;
use bevy_egui::egui;
use sugarfunge_api_types::validator::*;

pub mod add;
pub mod remove;

#[derive(Resource, Default)]
pub struct ValidatorUi {
    pub actions: ValidatorActions,
    pub data: ValidatorData,
    pub channels: ValidatorChannels,
}

#[derive(Resource, Debug, Default, Eq, PartialEq)]
pub enum ValidatorActions {
    #[default]
    AddValidator,
    RemoveValidator,
}

#[derive(Resource, Debug, Default, Clone)]
pub struct ValidatorInputData {
    add: add::AddValidatorInputData,
    remove: remove::RemoveValidatorInputData,
}

#[derive(Resource, Default, Debug)]
pub struct ValidatorOutputData {
    add: Option<AddValidatorOutput>,
    remove: Option<RemoveValidatorOutput>,
}

#[derive(Resource, Default)]
pub struct ValidatorData {
    input: ValidatorInputData,
    output: ValidatorOutputData,
}

#[derive(Resource, Default)]
pub struct ValidatorChannels {
    add: add::AddValidatorChannel,
    remove: remove::RemoveValidatorChannel,
}

pub fn validator_ui(ui: &mut egui::Ui, validator: &mut ResMut<ValidatorUi>) {
    ui.horizontal(|ui| {
        ui.selectable_value(
            &mut validator.actions,
            ValidatorActions::AddValidator,
            "Add",
        );
        ui.selectable_value(
            &mut validator.actions,
            ValidatorActions::RemoveValidator,
            "Remove",
        );
    });
    ui.separator();
    match &validator.actions {
        ValidatorActions::AddValidator => {
            add::add_validator_ui(ui, validator);
        }
        ValidatorActions::RemoveValidator => {
            remove::remove_validator_ui(ui, validator);
        }
    }
}

pub struct ValidatorPlugin;

impl Plugin for ValidatorPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ValidatorUi>()
            .add_plugin(add::AddValidatorPlugin)
            .add_plugin(remove::RemoveValidatorPlugin);
    }
}
