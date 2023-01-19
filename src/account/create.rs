use bevy::prelude::*;
use bevy_egui::egui;
use sugarfunge_api_types::account::CreateAccountOutput;

use crate::{
    prelude::*,
    util::{request_handler, setup_in_out_channels},
};

use super::{AccountInputData, AccountOutputData};

#[derive(Debug)]
pub struct CreateAccountRequest;

impl Request<()> for CreateAccountRequest {
    fn endpoint(&self) -> &str {
        "account/create"
    }

    fn input(&self) -> Option<()> {
        None
    }
}

#[derive(Resource, Debug, Default, Clone)]
pub struct CreateAccountInputData {
    pub loading: bool,
}

pub fn create_account_ui(
    ui: &mut egui::Ui,
    account_input: &mut ResMut<AccountInputData>,
    created_tx: &Res<InputSender<CreateAccountRequest>>,
    account_output: &Res<AccountOutputData>,
) {
    ui.label("Create Account");
    ui.separator();
    if account_input.create_input.loading {
        ui.separator();
        ui.add(egui::Spinner::default());
    } else {
        if ui.button("Create").clicked() {
            created_tx.0.send(CreateAccountRequest).unwrap();
            account_input.create_input.loading = true;
        }
    }
    if let Some(output) = &account_output.create_output {
        ui.separator();
        ui.label("Account");
        ui.text_edit_singleline(&mut output.account.as_str());
        ui.label("Seed");
        ui.text_edit_singleline(&mut output.seed.as_str());
    }
}

pub fn handle_create_response(
    mut account_output: ResMut<AccountOutputData>,
    mut account_input: ResMut<AccountInputData>,
    created_rx: Res<OutputReceiver<CreateAccountOutput>>,
) {
    if let Ok(created) = created_rx.0.try_recv() {
        account_output.create_output = Some(created);
        account_input.create_input.loading = false;
    }
}

pub struct AccountCreatePlugin;

impl Plugin for AccountCreatePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_in_out_channels::<CreateAccountRequest, CreateAccountOutput>)
            .add_system(request_handler::<CreateAccountRequest, (), CreateAccountOutput>)
            .add_system(handle_create_response);
    }
}
