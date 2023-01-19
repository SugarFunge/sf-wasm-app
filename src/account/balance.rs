use bevy::prelude::*;
use bevy_egui::egui;
use sugarfunge_api_types::{
    account::{AccountBalanceInput, AccountBalanceOutput},
    primitives::Account,
};

use crate::{
    prelude::*,
    util::{request_handler, setup_in_out_channels},
};

use super::{AccountInputData, AccountOutputData};

#[derive(Debug)]
pub struct AccountBalanceRequest {
    pub input: AccountBalanceInput,
}

impl Request<AccountBalanceInput> for AccountBalanceRequest {
    fn endpoint(&self) -> &str {
        "account/balance"
    }

    fn input(&self) -> Option<AccountBalanceInput> {
        Some(AccountBalanceInput {
            account: self.input.account.clone(),
        })
    }
}

#[derive(Resource, Default, Debug, Clone)]
pub struct AccountBalanceInputData {
    pub account: String,
    pub loading: bool,
}

pub fn account_balance_ui(
    ui: &mut egui::Ui,
    account_input: &mut ResMut<AccountInputData>,
    balance_tx: &Res<InputSender<AccountBalanceRequest>>,
    account_output: &Res<AccountOutputData>,
) {
    ui.label("Account Balance");
    ui.separator();
    ui.horizontal(|ui| {
        ui.label("Account");
        ui.text_edit_singleline(&mut account_input.balance_input.account);
    });
    if account_input.balance_input.loading {
        ui.separator();
        ui.add(egui::Spinner::default());
    } else {
        if ui.button("Check").clicked() {
            balance_tx
                .0
                .send(AccountBalanceRequest {
                    input: AccountBalanceInput {
                        account: Account::from(account_input.balance_input.account.clone()),
                    },
                })
                .unwrap();
            account_input.balance_input.loading = true;
        }
    }
    if let Some(output) = &account_output.balance_output {
        ui.separator();
        ui.label("Balance");
        ui.text_edit_singleline(&mut format!("{:?}", &output.balance));
    }
}

pub fn handle_balance_response(
    mut account_output: ResMut<AccountOutputData>,
    mut account_input: ResMut<AccountInputData>,
    balance_rx: Res<OutputReceiver<AccountBalanceOutput>>,
) {
    if let Ok(balance) = balance_rx.0.try_recv() {
        account_output.balance_output = Some(balance);
        account_input.balance_input.loading = false;
    }
}

pub struct AccountBalancePlugin;

impl Plugin for AccountBalancePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(
            setup_in_out_channels::<AccountBalanceRequest, AccountBalanceOutput>,
        )
        .add_system(
            request_handler::<AccountBalanceRequest, AccountBalanceInput, AccountBalanceOutput>,
        )
        .add_system(handle_balance_response);
    }
}
