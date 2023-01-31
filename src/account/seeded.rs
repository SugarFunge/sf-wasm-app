use bevy::prelude::*;
use bevy_egui::egui;
use crossbeam::channel;
use sugarfunge_api_types::{
    account::{SeededAccountInput, SeededAccountOutput},
    primitives::Seed,
};

use crate::{prelude::*, util::request_handler};

use super::AccountUi;

#[derive(Debug)]
pub struct SeededAccountRequest {
    pub input: SeededAccountInput,
}

impl Request<SeededAccountInput> for SeededAccountRequest {
    fn endpoint(&self) -> &str {
        "account/seeded"
    }

    fn input(&self) -> Option<SeededAccountInput> {
        Some(SeededAccountInput {
            seed: self.input.seed.clone(),
        })
    }
}

#[derive(Resource)]
pub struct SeededAccountChannel {
    pub input_tx: InputSender<SeededAccountRequest>,
    pub input_rx: InputReceiver<SeededAccountRequest>,
    pub output_tx: OutputSender<SeededAccountOutput>,
    pub output_rx: OutputReceiver<SeededAccountOutput>,
}

impl Default for SeededAccountChannel {
    fn default() -> Self {
        let (input_tx, input_rx) = channel::unbounded::<SeededAccountRequest>();
        let (output_tx, output_rx) = channel::unbounded::<Option<SeededAccountOutput>>();
        Self {
            input_tx: InputSender(input_tx),
            input_rx: InputReceiver(input_rx),
            output_tx: OutputSender(output_tx),
            output_rx: OutputReceiver(output_rx),
        }
    }
}

#[derive(Resource, Debug, Clone)]
pub struct SeededAccountInputData {
    pub seed: Seed,
    pub loading: bool,
}

impl Default for SeededAccountInputData {
    fn default() -> Self {
        Self {
            seed: Seed::from("".to_string()),
            loading: false,
        }
    }
}

pub fn seeded_account_ui(ui: &mut egui::Ui, account: &mut ResMut<AccountUi>) {
    ui.label("Seeded Account");
    ui.separator();
    ui.label("Seed");
    ui.text_edit_singleline(&mut *account.data.input.seeded.seed);
    if account.data.input.seeded.loading {
        ui.separator();
        ui.add(egui::Spinner::default());
    } else {
        if ui.button("Get Account from Seed").clicked() {
            account
                .channels
                .seeded
                .input_tx
                .0
                .send(SeededAccountRequest {
                    input: SeededAccountInput {
                        seed: account.data.input.seeded.seed.clone(),
                    },
                })
                .unwrap();
            account.data.input.seeded.loading = true;
        }
    }
    if let Some(output) = &account.data.output.seeded {
        ui.separator();
        ui.label("Account");
        ui.text_edit_singleline(&mut output.account.as_str());
        ui.label("Seed");
        ui.text_edit_singleline(&mut output.seed.as_str());
    }
}

pub fn handle_seeded_response(mut account: ResMut<AccountUi>, tokio_runtime: Res<TokioRuntime>) {
    if let Ok(seeded_result) = account.channels.seeded.output_rx.0.try_recv() {
        if let Some(seeded) = seeded_result {
            account.data.output.seeded = Some(seeded);
        }
        account.data.input.seeded.loading = false;
    }

    request_handler::<SeededAccountRequest, SeededAccountInput, SeededAccountOutput>(
        tokio_runtime.runtime.clone(),
        account.channels.seeded.input_rx.clone(),
        account.channels.seeded.output_tx.clone(),
    );
}

pub struct SeededAccountPlugin;

impl Plugin for SeededAccountPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(handle_seeded_response);
    }
}
