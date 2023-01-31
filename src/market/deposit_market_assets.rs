use bevy::prelude::*;
use bevy_egui::egui;
use crossbeam::channel;
use sugarfunge_api_types::{
    market::{DepositAssetsInput, DepositAssetsOutput, RateBalance},
    primitives::{Balance, MarketId, Seed},
};

use crate::{prelude::*, util::request_handler};

use super::MarketUi;

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

#[derive(Resource)]
pub struct DepositMarketAssetsChannel {
    pub input_tx: InputSender<DepositMarketAssetsRequest>,
    pub input_rx: InputReceiver<DepositMarketAssetsRequest>,
    pub output_tx: OutputSender<DepositAssetsOutput>,
    pub output_rx: OutputReceiver<DepositAssetsOutput>,
}

impl Default for DepositMarketAssetsChannel {
    fn default() -> Self {
        let (input_tx, input_rx) = channel::unbounded::<DepositMarketAssetsRequest>();
        let (output_tx, output_rx) = channel::unbounded::<Option<DepositAssetsOutput>>();
        Self {
            input_tx: InputSender(input_tx),
            input_rx: InputReceiver(input_rx),
            output_tx: OutputSender(output_tx),
            output_rx: OutputReceiver(output_rx),
        }
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

pub fn deposit_market_assets_ui(ui: &mut egui::Ui, market: &mut ResMut<MarketUi>) {
    ui.label("Deposit Market Assets");
    ui.separator();
    ui.label("Seed");
    ui.text_edit_singleline(&mut *market.data.input.deposit_market_assets.seed);
    ui.label("Market ID");
    ui.add(
        egui::DragValue::new(&mut *market.data.input.deposit_market_assets.market_id).speed(1.0),
    );
    ui.label("Market Rate ID");
    ui.add(
        egui::DragValue::new(&mut *market.data.input.deposit_market_assets.market_id).speed(1.0),
    );
    ui.label("Amount");
    ui.add(egui::DragValue::new(&mut market.data.input.deposit_market_assets.amount).speed(1.0));
    ui.separator();
    if market.data.input.deposit_market_assets.loading {
        ui.separator();
        ui.add(egui::Spinner::default());
    } else {
        if ui.button("Deposit").clicked() {
            market
                .channels
                .deposit_market_assets
                .input_tx
                .0
                .send(DepositMarketAssetsRequest {
                    input: DepositAssetsInput {
                        seed: market.data.input.deposit_market_assets.seed.clone(),
                        market_id: market.data.input.deposit_market_assets.market_id.clone(),
                        market_rate_id: market
                            .data
                            .input
                            .deposit_market_assets
                            .market_rate_id
                            .clone(),
                        amount: Balance::from(
                            market.data.input.deposit_market_assets.amount.clone() as u128,
                        ),
                    },
                })
                .unwrap();
            market.data.input.deposit_market_assets.loading = true;
        }
    }
    if let Some(output) = &market.data.output.deposit_market_assets {
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
    mut market: ResMut<MarketUi>,
    tokio_runtime: Res<TokioRuntime>,
) {
    if let Ok(deposited_result) = market.channels.deposit_market_assets.output_rx.0.try_recv() {
        if let Some(deposited) = deposited_result {
            market.data.output.deposit_market_assets = Some(deposited);
        }
        market.data.input.deposit_market_assets.loading = false;
    }

    request_handler::<DepositMarketAssetsRequest, DepositAssetsInput, DepositAssetsOutput>(
        tokio_runtime.runtime.clone(),
        market.channels.deposit_market_assets.input_rx.clone(),
        market.channels.deposit_market_assets.output_tx.clone(),
    );
}

pub struct DepositMarketAssetsPlugin;

impl Plugin for DepositMarketAssetsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(handle_deposit_market_assets_response);
    }
}
