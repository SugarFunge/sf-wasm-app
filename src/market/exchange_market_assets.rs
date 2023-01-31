use bevy::prelude::*;
use bevy_egui::egui;
use crossbeam::channel;
use sugarfunge_api_types::{
    market::{ExchangeAssetsInput, ExchangeAssetsOutput},
    primitives::{Balance, MarketId, Seed},
};

use crate::{prelude::*, util::request_handler};

use super::{deposit_market_assets::rate_balances_ui, MarketUi};

#[derive(Debug)]
pub struct ExchangeMarketAssetsRequest {
    pub input: ExchangeAssetsInput,
}

impl Request<ExchangeAssetsInput> for ExchangeMarketAssetsRequest {
    fn endpoint(&self) -> &str {
        "market/exchange_assets"
    }

    fn input(&self) -> Option<ExchangeAssetsInput> {
        Some(ExchangeAssetsInput {
            seed: self.input.seed.clone(),
            market_id: self.input.market_id.clone(),
            market_rate_id: self.input.market_rate_id.clone(),
            amount: self.input.amount.clone(),
        })
    }
}

#[derive(Resource)]
pub struct ExchangeMarketAssetsChannel {
    pub input_tx: InputSender<ExchangeMarketAssetsRequest>,
    pub input_rx: InputReceiver<ExchangeMarketAssetsRequest>,
    pub output_tx: OutputSender<ExchangeAssetsOutput>,
    pub output_rx: OutputReceiver<ExchangeAssetsOutput>,
}

impl Default for ExchangeMarketAssetsChannel {
    fn default() -> Self {
        let (input_tx, input_rx) = channel::unbounded::<ExchangeMarketAssetsRequest>();
        let (output_tx, output_rx) = channel::unbounded::<Option<ExchangeAssetsOutput>>();
        Self {
            input_tx: InputSender(input_tx),
            input_rx: InputReceiver(input_rx),
            output_tx: OutputSender(output_tx),
            output_rx: OutputReceiver(output_rx),
        }
    }
}

#[derive(Resource, Debug, Clone)]
pub struct ExchangeMarketAssetsInputData {
    pub seed: Seed,
    pub market_id: MarketId,
    pub market_rate_id: MarketId,
    pub amount: u64,
    pub loading: bool,
}

impl Default for ExchangeMarketAssetsInputData {
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

pub fn exchange_market_assets_ui(ui: &mut egui::Ui, market: &mut ResMut<MarketUi>) {
    ui.label("Exchange Market Assets");
    ui.separator();
    ui.label("Seed");
    ui.text_edit_singleline(&mut *market.data.input.exchange_market_assets.seed);
    ui.label("Market ID");
    ui.add(
        egui::DragValue::new(&mut *market.data.input.exchange_market_assets.market_id).speed(1.0),
    );
    ui.label("Market Rate ID");
    ui.add(
        egui::DragValue::new(&mut *market.data.input.exchange_market_assets.market_rate_id)
            .speed(1.0),
    );
    ui.label("Amount");
    ui.add(egui::DragValue::new(&mut market.data.input.exchange_market_assets.amount).speed(1.0));
    ui.separator();
    if market.data.input.exchange_market_assets.loading {
        ui.separator();
        ui.add(egui::Spinner::default());
    } else {
        if ui.button("Exchange").clicked() {
            market
                .channels
                .exchange_market_assets
                .input_tx
                .0
                .send(ExchangeMarketAssetsRequest {
                    input: ExchangeAssetsInput {
                        seed: market.data.input.exchange_market_assets.seed.clone(),
                        market_id: market.data.input.exchange_market_assets.market_id.clone(),
                        market_rate_id: market
                            .data
                            .input
                            .exchange_market_assets
                            .market_rate_id
                            .clone(),
                        amount: Balance::from(
                            market.data.input.exchange_market_assets.amount.clone() as u128,
                        ),
                    },
                })
                .unwrap();
            market.data.input.exchange_market_assets.loading = true;
        }
    }

    if let Some(output) = &market.data.output.exchange_market_assets {
        ui.label("Buyer");
        ui.text_edit_singleline(&mut output.buyer.to_string());
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

pub fn handle_exchange_market_assets_response(
    mut market: ResMut<MarketUi>,
    tokio_runtime: Res<TokioRuntime>,
) {
    if let Ok(exchanged_result) = market
        .channels
        .exchange_market_assets
        .output_rx
        .0
        .try_recv()
    {
        if let Some(exchanged) = exchanged_result {
            market.data.output.exchange_market_assets = Some(exchanged);
        }
        market.data.input.exchange_market_assets.loading = false;
    }

    request_handler::<ExchangeMarketAssetsRequest, ExchangeAssetsInput, ExchangeAssetsOutput>(
        tokio_runtime.runtime.clone(),
        market.channels.exchange_market_assets.input_rx.clone(),
        market.channels.exchange_market_assets.output_tx.clone(),
    );
}

pub struct ExchangeMarketAssetsPlugin;

impl Plugin for ExchangeMarketAssetsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(handle_exchange_market_assets_response);
    }
}
