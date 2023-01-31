use bevy::prelude::*;
use bevy_egui::egui;
use crossbeam::channel;
use sugarfunge_api_types::{
    account::{AccountExistsInput, AccountExistsOutput},
    primitives::Account,
};

use crate::{prelude::*, util::request_handler};

use super::AccountUi;

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

#[derive(Resource)]
pub struct AccountExistsChannel {
    pub input_tx: InputSender<AccountExistsRequest>,
    pub input_rx: InputReceiver<AccountExistsRequest>,
    pub output_tx: OutputSender<AccountExistsOutput>,
    pub output_rx: OutputReceiver<AccountExistsOutput>,
}

impl Default for AccountExistsChannel {
    fn default() -> Self {
        let (input_tx, input_rx) = channel::unbounded::<AccountExistsRequest>();
        let (output_tx, output_rx) = channel::unbounded::<Option<AccountExistsOutput>>();
        Self {
            input_tx: InputSender(input_tx),
            input_rx: InputReceiver(input_rx),
            output_tx: OutputSender(output_tx),
            output_rx: OutputReceiver(output_rx),
        }
    }
}

#[derive(Resource, Debug, Clone)]
pub struct AccountExistsInputData {
    pub account: Account,
    pub loading: bool,
}

impl Default for AccountExistsInputData {
    fn default() -> Self {
        Self {
            account: Account::from("".to_string()),
            loading: false,
        }
    }
}

pub fn account_exists_ui(ui: &mut egui::Ui, account: &mut ResMut<AccountUi>) {
    ui.label("Account Exists");
    ui.separator();
    ui.label("Account");
    ui.text_edit_singleline(&mut *account.data.input.exists.account);
    if account.data.input.exists.loading {
        ui.separator();
        ui.add(egui::Spinner::default());
    } else {
        if ui.button("Check").clicked() {
            account
                .channels
                .exists
                .input_tx
                .0
                .send(AccountExistsRequest {
                    input: AccountExistsInput {
                        account: account.data.input.exists.account.clone(),
                    },
                })
                .unwrap();
            account.data.input.exists.loading = true;
        }
    }
    if let Some(output) = &account.data.output.exists {
        ui.separator();
        ui.label("Exists");
        ui.text_edit_singleline(&mut output.exists.to_string());
    }
}

pub fn handle_exists_response(mut account: ResMut<AccountUi>, tokio_runtime: Res<TokioRuntime>) {
    if let Ok(exists_result) = account.channels.exists.output_rx.0.try_recv() {
        if let Some(exists) = exists_result {
            account.data.output.exists = Some(exists);
        }
        account.data.input.exists.loading = false;
    }

    request_handler::<AccountExistsRequest, AccountExistsInput, AccountExistsOutput>(
        tokio_runtime.runtime.clone(),
        account.channels.exists.input_rx.clone(),
        account.channels.exists.output_tx.clone(),
    );
}

pub struct AccountExistsPlugin;

impl Plugin for AccountExistsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(handle_exists_response);
    }
}
