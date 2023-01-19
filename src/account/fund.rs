use bevy::prelude::*;
use bevy_egui::egui;
use sugarfunge_api_types::{
    account::{FundAccountInput, FundAccountOutput},
    primitives::{Account, Balance, Seed},
};

use crate::{
    prelude::*,
    util::{request_handler, setup_in_out_channels},
};

use super::{AccountInputData, AccountOutputData};

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

#[derive(Resource, Debug, Clone)]
pub struct FundAccountInputData {
    pub seed: String,
    pub to: String,
    pub amount: u64,
}

impl Default for FundAccountInputData {
    fn default() -> Self {
        Self {
            seed: "//Alice".to_string(),
            to: "".to_string(),
            amount: 1,
        }
    }
}

pub fn account_fund_ui(
    ui: &mut egui::Ui,
    account_input: &mut ResMut<AccountInputData>,
    funded_tx: &Res<InputSender<FundAccountRequest>>,
    account_output: &Res<AccountOutputData>,
) {
    ui.label("Fund Account");
    ui.label("Seed");
    ui.text_edit_singleline(&mut account_input.fund_account.seed);
    ui.label("To");
    ui.text_edit_singleline(&mut account_input.fund_account.to);
    ui.label("Amount");
    ui.label("The Amount is represented in 10^18 units.");
    ui.add(egui::DragValue::new::<u64>(&mut account_input.fund_account.amount.into()).speed(0.1));
    if ui.button("Fund").clicked() {
        funded_tx
            .0
            .send(FundAccountRequest {
                input: FundAccountInput {
                    seed: Seed::from(account_input.fund_account.seed.clone()),
                    to: Account::from(account_input.fund_account.to.clone()),
                    amount: Balance::from(
                        (account_input.fund_account.amount as u128) * (u128::pow(10, 18)),
                    ),
                },
            })
            .unwrap();
    }
    if let Some(funded) = &account_output.fund_account {
        ui.separator();
        ui.label("From");
        ui.text_edit_singleline(&mut funded.from.as_str());
        ui.label("To");
        ui.text_edit_singleline(&mut funded.to.as_str());
        ui.label("Amount");
        ui.text_edit_singleline(&mut format!("{:?}", &funded.amount));
    }
}

pub struct AccountFundPlugin;

impl Plugin for AccountFundPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_in_out_channels::<FundAccountRequest, FundAccountOutput>)
            .add_system(request_handler::<FundAccountRequest, FundAccountInput, FundAccountOutput>);
    }
}
