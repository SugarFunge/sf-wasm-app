use bevy::prelude::*;
use bevy_egui::egui;
use sugarfunge_api_types::{
    account::{AccountExistsInput, AccountExistsOutput},
    primitives::Account,
};

use crate::{
    prelude::*,
    util::{request_handler, setup_in_out_channels},
};

use super::{AccountInputData, AccountOutputData};

#[derive(Debug)]
pub struct AccountExistsRequest {
    pub input: AccountExistsInput,
}

impl Request<AccountExistsInput> for AccountExistsRequest {
    fn endpoint(&self) -> &str {
        "account/exists"
    }

    fn input(&self) -> Option<AccountExistsInput> {
        Some(AccountExistsInput {
            account: self.input.account.clone(),
        })
    }
}

#[derive(Resource, Default, Debug, Clone)]
pub struct AccountExistsInputData {
    pub account: String,
    pub loading: bool,
}

pub fn account_exists_ui(
    ui: &mut egui::Ui,
    account_input: &mut ResMut<AccountInputData>,
    exists_tx: &Res<InputSender<AccountExistsRequest>>,
    account_output: &Res<AccountOutputData>,
) {
    ui.label("Account Exists");
    ui.separator();
    ui.label("Account");
    ui.text_edit_singleline(&mut account_input.exists_input.account);
    if account_input.exists_input.loading {
        ui.separator();
        ui.add(egui::Spinner::default());
    } else {
        if ui.button("Check").clicked() {
            exists_tx
                .0
                .send(AccountExistsRequest {
                    input: AccountExistsInput {
                        account: Account::from(account_input.exists_input.account.clone()),
                    },
                })
                .unwrap();
            account_input.exists_input.loading = true;
        }
    }
    if let Some(output) = &account_output.exists_output {
        ui.separator();
        ui.label("Exists");
        ui.text_edit_singleline(&mut output.exists.to_string());
    }
}

pub fn handle_exists_response(
    mut account_output: ResMut<AccountOutputData>,
    mut account_input: ResMut<AccountInputData>,
    exists_rx: Res<OutputReceiver<AccountExistsOutput>>,
) {
    if let Ok(exists_result) = exists_rx.0.try_recv() {
        if let Some(exists) = exists_result {
            account_output.exists_output = Some(exists);
        }
        account_input.exists_input.loading = false;
    }
}

pub struct AccountExistsPlugin;

impl Plugin for AccountExistsPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_in_out_channels::<AccountExistsRequest, AccountExistsOutput>)
            .add_system(
                request_handler::<AccountExistsRequest, AccountExistsInput, AccountExistsOutput>,
            )
            .add_system(handle_exists_response);
    }
}
