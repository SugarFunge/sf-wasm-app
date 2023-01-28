use bevy::prelude::*;
use bevy_egui::egui;
use sugarfunge_api_types::{
    market::{ExchangeAssetsInput, ExchangeAssetsOutput},
    primitives::{Balance, MarketId, Seed},
};

use crate::{prelude::*, util::*};

use super::{deposit_market_assets::rate_balances_ui, MarketInputData, MarketOutputData};

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

pub fn exchange_market_assets_ui(
    ui: &mut egui::Ui,
    market_input: &mut ResMut<MarketInputData>,
    exchange_tx: &Res<InputSender<ExchangeMarketAssetsRequest>>,
    market_output: &Res<MarketOutputData>,
) {
    ui.label("Exchange Market Assets");
    ui.separator();
    ui.label("Seed");
    ui.text_edit_singleline(&mut *market_input.exchange_market_assets_input.seed);
    ui.label("Market ID");
    ui.add(
        egui::DragValue::new(&mut *market_input.exchange_market_assets_input.market_id).speed(1.0),
    );
    ui.label("Market Rate ID");
    ui.add(
        egui::DragValue::new(&mut *market_input.exchange_market_assets_input.market_rate_id)
            .speed(1.0),
    );
    ui.label("Amount");
    ui.add(egui::DragValue::new(&mut market_input.exchange_market_assets_input.amount).speed(1.0));
    ui.separator();
    if market_input.exchange_market_assets_input.loading {
        ui.separator();
        ui.add(egui::Spinner::default());
    } else {
        if ui.button("Exchange").clicked() {
            exchange_tx
                .send(ExchangeMarketAssetsRequest {
                    input: ExchangeAssetsInput {
                        seed: market_input.exchange_market_assets_input.seed.clone(),
                        market_id: market_input.exchange_market_assets_input.market_id.clone(),
                        market_rate_id: market_input
                            .exchange_market_assets_input
                            .market_rate_id
                            .clone(),
                        amount: Balance::from(
                            market_input.exchange_market_assets_input.amount.clone() as u128,
                        ),
                    },
                })
                .unwrap();
            market_input.exchange_market_assets_input.loading = true;
        }
    }

    if let Some(output) = &market_output.exchange_market_assets_output {
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
    mut market_output: ResMut<MarketOutputData>,
    mut market_input: ResMut<MarketInputData>,
    exchanged_rx: Res<OutputReceiver<ExchangeAssetsOutput>>,
) {
    if let Ok(exchanged_result) = exchanged_rx.0.try_recv() {
        if let Some(exchanged) = exchanged_result {
            market_output.exchange_market_assets_output = Some(exchanged);
        }
        market_input.exchange_market_assets_input.loading = false;
    }
}

pub struct ExchangeMarketAssetsPlugin;

impl Plugin for ExchangeMarketAssetsPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(
            setup_in_out_channels::<ExchangeMarketAssetsRequest, ExchangeAssetsOutput>,
        )
        .add_system(
            request_handler::<ExchangeMarketAssetsRequest, ExchangeAssetsInput, ExchangeAssetsOutput>,
        )
        .add_system(handle_exchange_market_assets_response);
    }
}
