use bevy::prelude::*;
use bevy_egui::egui;
use sugarfunge_api_types::account::CreateAccountOutput;

use crate::{
    prelude::*,
    util::{request_handler, setup_in_out_channels},
};

use super::AccountOutputData;

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

pub fn create_account_ui(
    ui: &mut egui::Ui,
    created_tx: &Res<InputSender<CreateAccountRequest>>,
    account_output: &Res<AccountOutputData>,
) {
    ui.label("Create Account");
    if ui.button("Create").clicked() {
        created_tx.0.send(CreateAccountRequest).unwrap();
    }
    if let Some(created) = &account_output.create_account {
        ui.separator();
        ui.label("Account");
        ui.text_edit_singleline(&mut created.account.as_str());
        ui.label("Seed");
        ui.text_edit_singleline(&mut created.seed.as_str());
    }
}

pub struct AccountCreatePlugin;

impl Plugin for AccountCreatePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_in_out_channels::<CreateAccountRequest, CreateAccountOutput>)
            .add_system(request_handler::<CreateAccountRequest, (), CreateAccountOutput>);
    }
}
