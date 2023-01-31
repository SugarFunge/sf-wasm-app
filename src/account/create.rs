use bevy::prelude::*;
use bevy_egui::egui;
use crossbeam::channel;
use sugarfunge_api_types::account::CreateAccountOutput;

use crate::{prelude::*, util::request_handler};

use super::AccountUi;

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

#[derive(Resource)]
pub struct CreateAccountChannel {
    pub input_tx: InputSender<CreateAccountRequest>,
    pub input_rx: InputReceiver<CreateAccountRequest>,
    pub output_tx: OutputSender<CreateAccountOutput>,
    pub output_rx: OutputReceiver<CreateAccountOutput>,
}

impl Default for CreateAccountChannel {
    fn default() -> Self {
        let (input_tx, input_rx) = channel::unbounded::<CreateAccountRequest>();
        let (output_tx, output_rx) = channel::unbounded::<Option<CreateAccountOutput>>();
        Self {
            input_tx: InputSender(input_tx),
            input_rx: InputReceiver(input_rx),
            output_tx: OutputSender(output_tx),
            output_rx: OutputReceiver(output_rx),
        }
    }
}

#[derive(Resource, Debug, Default, Clone)]
pub struct CreateAccountInputData {
    pub loading: bool,
}

pub fn create_account_ui(ui: &mut egui::Ui, account: &mut ResMut<AccountUi>) {
    ui.label("Create Account");
    ui.separator();
    if account.data.input.create.loading {
        ui.separator();
        ui.add(egui::Spinner::default());
    } else {
        if ui.button("Create").clicked() {
            account
                .channels
                .create
                .input_tx
                .0
                .send(CreateAccountRequest)
                .unwrap();
            account.data.input.create.loading = true;
        }
    }
    if let Some(output) = &account.data.output.create {
        ui.separator();
        ui.label("Account");
        ui.text_edit_singleline(&mut output.account.as_str());
        ui.label("Seed");
        ui.text_edit_singleline(&mut output.seed.as_str());
    }
}

pub fn handle_create_response(mut account: ResMut<AccountUi>, tokio_runtime: Res<TokioRuntime>) {
    if let Ok(created_result) = account.channels.create.output_rx.0.try_recv() {
        if let Some(created) = created_result {
            account.data.output.create = Some(created);
        }
        account.data.input.create.loading = false;
    }

    request_handler::<CreateAccountRequest, (), CreateAccountOutput>(
        tokio_runtime.runtime.clone(),
        account.channels.create.input_rx.clone(),
        account.channels.create.output_tx.clone(),
    );
}

pub struct AccountCreatePlugin;

impl Plugin for AccountCreatePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(handle_create_response);
    }
}
