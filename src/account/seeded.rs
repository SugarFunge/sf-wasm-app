use bevy::prelude::*;
use bevy_egui::egui;
use sugarfunge_api_types::{
    account::{SeededAccountInput, SeededAccountOutput},
    primitives::Seed,
};

use crate::{
    prelude::*,
    util::{request_handler, setup_in_out_channels},
};

use super::{AccountInputData, AccountOutputData};

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

#[derive(Resource, Default, Debug, Clone)]
pub struct SeededAccountInputData {
    pub seed: String,
    pub loading: bool,
}

pub fn seeded_account_ui(
    ui: &mut egui::Ui,
    account_input: &mut ResMut<AccountInputData>,
    seeded_tx: &Res<InputSender<SeededAccountRequest>>,
    account_output: &Res<AccountOutputData>,
) {
    ui.label("Seeded Account");
    ui.separator();
    ui.label("Seed");
    ui.text_edit_singleline(&mut account_input.seeded_input.seed);
    if account_input.seeded_input.loading {
        ui.separator();
        ui.add(egui::Spinner::default());
    } else {
        if ui.button("Get Account from Seed").clicked() {
            seeded_tx
                .0
                .send(SeededAccountRequest {
                    input: SeededAccountInput {
                        seed: Seed::from(account_input.seeded_input.seed.clone()),
                    },
                })
                .unwrap();
            account_input.seeded_input.loading = true;
        }
    }
    if let Some(output) = &account_output.seeded_output {
        ui.separator();
        ui.label("Account");
        ui.text_edit_singleline(&mut output.account.as_str());
        ui.label("Seed");
        ui.text_edit_singleline(&mut output.seed.as_str());
    }
}

pub fn handle_seeded_response(
    mut account_output: ResMut<AccountOutputData>,
    mut account_input: ResMut<AccountInputData>,
    seeded_rx: Res<OutputReceiver<SeededAccountOutput>>,
) {
    if let Ok(seeded) = seeded_rx.0.try_recv() {
        account_output.seeded_output = Some(seeded);
        account_input.seeded_input.loading = false;
    }
}

pub struct SeededAccountPlugin;

impl Plugin for SeededAccountPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_in_out_channels::<SeededAccountRequest, SeededAccountOutput>)
            .add_system(
                request_handler::<SeededAccountRequest, SeededAccountInput, SeededAccountOutput>,
            )
            .add_system(handle_seeded_response);
    }
}
