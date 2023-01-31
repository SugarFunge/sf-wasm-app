use bevy::prelude::*;
use bevy_egui::egui;
use crossbeam::channel;
use sugarfunge_api_types::{
    account::{AccountBalanceInput, AccountBalanceOutput},
    primitives::Account,
};

use crate::{prelude::*, util::request_handler};

use super::AccountUi;

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

#[derive(Resource)]
pub struct AccountBalanceChannel {
    pub input_tx: InputSender<AccountBalanceRequest>,
    pub input_rx: InputReceiver<AccountBalanceRequest>,
    pub output_tx: OutputSender<AccountBalanceOutput>,
    pub output_rx: OutputReceiver<AccountBalanceOutput>,
}

impl Default for AccountBalanceChannel {
    fn default() -> Self {
        let (input_tx, input_rx) = channel::unbounded::<AccountBalanceRequest>();
        let (output_tx, output_rx) = channel::unbounded::<Option<AccountBalanceOutput>>();
        Self {
            input_tx: InputSender(input_tx),
            input_rx: InputReceiver(input_rx),
            output_tx: OutputSender(output_tx),
            output_rx: OutputReceiver(output_rx),
        }
    }
}

#[derive(Resource, Debug, Clone)]
pub struct AccountBalanceInputData {
    pub account: Account,
    pub loading: bool,
}

impl Default for AccountBalanceInputData {
    fn default() -> Self {
        Self {
            account: Account::from("".to_string()),
            loading: false,
        }
    }
}

pub fn account_balance_ui(ui: &mut egui::Ui, account: &mut ResMut<AccountUi>) {
    ui.label("Account Balance");
    ui.separator();
    ui.horizontal(|ui| {
        ui.label("Account");
        ui.text_edit_singleline(&mut *account.data.input.balance.account);
    });
    if account.data.input.balance.loading {
        ui.separator();
        ui.add(egui::Spinner::default());
    } else {
        if ui.button("Check").clicked() {
            account
                .channels
                .balance
                .input_tx
                .0
                .send(AccountBalanceRequest {
                    input: AccountBalanceInput {
                        account: account.data.input.balance.account.clone(),
                    },
                })
                .unwrap();
            account.data.input.balance.loading = true;
        }
    }
    if let Some(output) = &account.data.output.balance {
        ui.separator();
        ui.label("Balance");
        ui.text_edit_singleline(&mut u128::from(output.balance).to_string());
    }
}

pub fn handle_balance_response(mut account: ResMut<AccountUi>, tokio_runtime: Res<TokioRuntime>) {
    if let Ok(balance_result) = account.channels.balance.output_rx.0.try_recv() {
        if let Some(balance) = balance_result {
            account.data.output.balance = Some(balance);
        }
        account.data.input.balance.loading = false;
    }

    request_handler::<AccountBalanceRequest, AccountBalanceInput, AccountBalanceOutput>(
        tokio_runtime.runtime.clone(),
        account.channels.balance.input_rx.clone(),
        account.channels.balance.output_tx.clone(),
    );
}

pub struct AccountBalancePlugin;

impl Plugin for AccountBalancePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(handle_balance_response);
    }
}
