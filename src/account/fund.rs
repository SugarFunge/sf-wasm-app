use bevy::prelude::*;
use bevy_egui::egui;
use crossbeam::channel;
use sugarfunge_api_types::{
    account::{FundAccountInput, FundAccountOutput},
    primitives::{Account, Balance, Seed},
};

use crate::{prelude::*, util::request_handler};

use super::AccountUi;

#[derive(Debug)]
pub struct FundAccountRequest {
    input: FundAccountInput,
}

impl Request<FundAccountInput> for FundAccountRequest {
    fn endpoint(&self) -> &str {
        "account/fund"
    }

    fn input(&self) -> Option<FundAccountInput> {
        Some(FundAccountInput {
            seed: self.input.seed.clone(),
            to: self.input.to.clone(),
            amount: self.input.amount,
        })
    }
}

#[derive(Resource)]
pub struct FundAccountChannel {
    pub input_tx: InputSender<FundAccountRequest>,
    pub input_rx: InputReceiver<FundAccountRequest>,
    pub output_tx: OutputSender<FundAccountOutput>,
    pub output_rx: OutputReceiver<FundAccountOutput>,
}

impl Default for FundAccountChannel {
    fn default() -> Self {
        let (input_tx, input_rx) = channel::unbounded::<FundAccountRequest>();
        let (output_tx, output_rx) = channel::unbounded::<Option<FundAccountOutput>>();
        Self {
            input_tx: InputSender(input_tx),
            input_rx: InputReceiver(input_rx),
            output_tx: OutputSender(output_tx),
            output_rx: OutputReceiver(output_rx),
        }
    }
}

#[derive(Resource, Debug, Clone)]
pub struct FundAccountInputData {
    pub seed: Seed,
    pub to: Account,
    pub amount: u64,
    pub loading: bool,
}

impl Default for FundAccountInputData {
    fn default() -> Self {
        Self {
            seed: Seed::from("//Alice".to_string()),
            to: Account::from("".to_string()),
            amount: 1,
            loading: false,
        }
    }
}

pub fn account_fund_ui(ui: &mut egui::Ui, account: &mut ResMut<AccountUi>) {
    ui.label("Fund Account");
    ui.separator();
    ui.label("Seed");
    ui.text_edit_singleline(&mut *account.data.input.fund.seed);
    ui.label("To");
    ui.text_edit_singleline(&mut *account.data.input.fund.to);
    ui.label("Amount");
    ui.label("The Amount is represented in 10^18 units.");
    ui.add(egui::DragValue::new::<u64>(&mut account.data.input.fund.amount.into()).speed(0.1));
    if account.data.input.fund.loading {
        ui.separator();
        ui.add(egui::Spinner::default());
    } else {
        if ui.button("Fund").clicked() {
            account
                .channels
                .fund
                .input_tx
                .0
                .send(FundAccountRequest {
                    input: FundAccountInput {
                        seed: account.data.input.fund.seed.clone(),
                        to: account.data.input.fund.to.clone(),
                        amount: Balance::from(
                            (account.data.input.fund.amount as u128) * (u128::pow(10, 18)),
                        ),
                    },
                })
                .unwrap();
            account.data.input.fund.loading = true;
        }
    }
    if let Some(output) = &account.data.output.fund {
        ui.separator();
        ui.label("From");
        ui.text_edit_singleline(&mut output.from.as_str());
        ui.label("To");
        ui.text_edit_singleline(&mut output.to.as_str());
        ui.label("Amount");
        ui.text_edit_singleline(&mut u128::from(output.amount).to_string());
    }
}

fn handle_funded_response(mut account: ResMut<AccountUi>, tokio_runtime: Res<TokioRuntime>) {
    if let Ok(funded_result) = account.channels.fund.output_rx.0.try_recv() {
        if let Some(funded) = funded_result {
            account.data.output.fund = Some(funded);
        }
        account.data.input.fund.loading = false;
    }

    request_handler::<FundAccountRequest, FundAccountInput, FundAccountOutput>(
        tokio_runtime.runtime.clone(),
        account.channels.fund.input_rx.clone(),
        account.channels.fund.output_tx.clone(),
    );
}

pub struct AccountFundPlugin;

impl Plugin for AccountFundPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(handle_funded_response);
    }
}
