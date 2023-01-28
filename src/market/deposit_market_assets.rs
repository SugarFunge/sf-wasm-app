use bevy::prelude::*;
use bevy_egui::egui;
use sugarfunge_api_types::{
    market::{DepositAssetsInput, DepositAssetsOutput, RateBalance},
    primitives::{Balance, MarketId, Seed},
};

use crate::{prelude::*, util::*};

use super::{MarketInputData, MarketOutputData};

pub struct DepositMarketAssetsRequest {
    pub input: DepositAssetsInput,
}

impl Request<DepositAssetsInput> for DepositMarketAssetsRequest {
    fn endpoint(&self) -> &str {
        "market/deposit_assets"
    }
    fn input(&self) -> Option<DepositAssetsInput> {
        Some(DepositAssetsInput {
            seed: self.input.seed.clone(),
            market_id: self.input.market_id.clone(),
            market_rate_id: self.input.market_rate_id.clone(),
            amount: self.input.amount.clone(),
        })
    }
}

#[derive(Resource, Debug, Clone)]
pub struct DepositMarketAssetsInputData {
    pub seed: Seed,
    pub market_id: MarketId,
    pub market_rate_id: MarketId,
    pub amount: u64,
    pub loading: bool,
}

impl Default for DepositMarketAssetsInputData {
    fn default() -> Self {
        Self {
            seed: Seed::from("".to_string()),
            market_id: MarketId::from(0),
            market_rate_id: MarketId::from(0),
            amount: 0,
            loading: false,
        }
    }
}

pub fn rate_balances_ui(ui: &mut egui::Ui, rate_balances: &Vec<RateBalance>) {
    ui.label("Balances");
    for (i, rate_balance) in rate_balances.iter().enumerate() {
        ui.label(format!("Rate Balance [{}]", i + 1));
        ui.label("Balance");
        ui.text_edit_singleline(&mut i128::from(rate_balance.balance).to_string());
        ui.label("Class ID");
        ui.text_edit_singleline(&mut u64::from(rate_balance.rate.class_id).to_string());
        ui.label("Asset ID");
        ui.text_edit_singleline(&mut u64::from(rate_balance.rate.asset_id).to_string());
        ui.label("Action");
        ui.text_edit_singleline(&mut format!("{:?}", rate_balance.rate.action));
        ui.label("From");
        ui.text_edit_singleline(&mut format!("{:?}", rate_balance.rate.from));
        ui.label("To");
        ui.text_edit_singleline(&mut format!("{:?}", rate_balance.rate.to));
        ui.separator();
    }
}

pub fn deposit_market_assets_ui(
    ui: &mut egui::Ui,
    market_input: &mut ResMut<MarketInputData>,
    deposit_tx: &Res<InputSender<DepositMarketAssetsRequest>>,
    market_output: &Res<MarketOutputData>,
) {
    ui.label("Deposit Market Assets");
    ui.separator();
    ui.label("Seed");
    ui.text_edit_singleline(&mut *market_input.deposit_market_assets_input.seed);
    ui.label("Market ID");
    ui.add(
        egui::DragValue::new(&mut *market_input.deposit_market_assets_input.market_id).speed(1.0),
    );
    ui.label("Market Rate ID");
    ui.add(
        egui::DragValue::new(&mut *market_input.deposit_market_assets_input.market_id).speed(1.0),
    );
    ui.label("Amount");
    ui.add(egui::DragValue::new(&mut market_input.deposit_market_assets_input.amount).speed(1.0));
    ui.separator();
    if market_input.deposit_market_assets_input.loading {
        ui.separator();
        ui.add(egui::Spinner::default());
    } else {
        if ui.button("Deposit").clicked() {
            deposit_tx
                .send(DepositMarketAssetsRequest {
                    input: DepositAssetsInput {
                        seed: market_input.deposit_market_assets_input.seed.clone(),
                        market_id: market_input.deposit_market_assets_input.market_id.clone(),
                        market_rate_id: market_input
                            .deposit_market_assets_input
                            .market_rate_id
                            .clone(),
                        amount: Balance::from(
                            market_input.deposit_market_assets_input.amount.clone() as u128,
                        ),
                    },
                })
                .unwrap();
            market_input.deposit_market_assets_input.loading = true;
        }
    }
    if let Some(output) = &market_output.deposit_market_assets_output {
        ui.separator();
        ui.label("Who");
        ui.text_edit_singleline(&mut output.who.to_string());
        ui.label("Market ID");
        ui.text_edit_singleline(&mut u64::from(output.market_id).to_string());
        ui.label("Market Rate ID");
        ui.text_edit_singleline(&mut u64::from(output.market_rate_id).to_string());
        ui.label("Amount");
        ui.text_edit_singleline(&mut u128::from(output.amount).to_string());
        rate_balances_ui(ui, &output.balances);
        ui.label("Success");
        ui.text_edit_singleline(&mut output.success.to_string());
    }
}

pub fn handle_deposit_market_assets_response(
    mut market_output: ResMut<MarketOutputData>,
    mut market_input: ResMut<MarketInputData>,
    deposited_rx: Res<OutputReceiver<DepositAssetsOutput>>,
) {
    if let Ok(deposited_result) = deposited_rx.0.try_recv() {
        if let Some(deposited) = deposited_result {
            market_output.deposit_market_assets_output = Some(deposited);
        }
        market_input.deposit_market_assets_input.loading = false;
    }
}

pub struct DepositMarketAssetsPlugin;

impl Plugin for DepositMarketAssetsPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(
            setup_in_out_channels::<DepositMarketAssetsRequest, DepositAssetsOutput>,
        )
        .add_system(
            request_handler::<DepositMarketAssetsRequest, DepositAssetsInput, DepositAssetsOutput>,
        )
        .add_system(handle_deposit_market_assets_response);
    }
}
