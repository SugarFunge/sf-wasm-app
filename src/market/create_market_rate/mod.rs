use bevy::prelude::*;
use bevy_egui::egui;
use crossbeam::channel;
use sugarfunge_api_types::{
    market::{
        AssetRate, CreateMarketRateInput, CreateMarketRateOutput, RateAccount, RateAction, Rates,
        AMM,
    },
    primitives::{Amount, AssetId, ClassId, MarketId, Seed},
};

use crate::{prelude::*, util::request_handler};

use self::{rate_account::*, rate_action::*};

use super::MarketUi;

pub mod rate_account;
pub mod rate_action;

#[derive(Debug)]
pub struct CreateMarketRateRequest {
    pub input: CreateMarketRateInput,
}

impl Request<CreateMarketRateInput> for CreateMarketRateRequest {
    fn endpoint(&self) -> &str {
        "market/create_market_rate"
    }

    fn input(&self) -> Option<CreateMarketRateInput> {
        Some(CreateMarketRateInput {
            seed: self.input.seed.clone(),
            market_id: self.input.market_id.clone(),
            market_rate_id: self.input.market_rate_id.clone(),
            rates: Rates {
                rates: self.input.rates.rates.clone(),
                metadata: self.input.rates.metadata.clone(),
            },
        })
    }
}

#[derive(Resource)]
pub struct CreateMarketRateChannel {
    pub input_tx: InputSender<CreateMarketRateRequest>,
    pub input_rx: InputReceiver<CreateMarketRateRequest>,
    pub output_tx: OutputSender<CreateMarketRateOutput>,
    pub output_rx: OutputReceiver<CreateMarketRateOutput>,
}

impl Default for CreateMarketRateChannel {
    fn default() -> Self {
        let (input_tx, input_rx) = channel::unbounded::<CreateMarketRateRequest>();
        let (output_tx, output_rx) = channel::unbounded::<Option<CreateMarketRateOutput>>();
        Self {
            input_tx: InputSender(input_tx),
            input_rx: InputReceiver(input_rx),
            output_tx: OutputSender(output_tx),
            output_rx: OutputReceiver(output_rx),
        }
    }
}

#[derive(Resource, Debug, Clone)]
pub struct MarketRateInputData {
    pub class_id: ClassId,
    pub asset_id: AssetId,
    pub action_ui: MarketRateAction,
    pub action_data: MarketRateActionInputData,
    pub from: MarketRateAccountInputData,
    pub to: MarketRateAccountInputData,
}

impl Default for MarketRateInputData {
    fn default() -> Self {
        Self {
            class_id: ClassId::from(0),
            asset_id: AssetId::from(0),
            action_ui: MarketRateAction::Transfer,
            action_data: MarketRateActionInputData::default(),
            from: MarketRateAccountInputData::default(),
            to: MarketRateAccountInputData::default(),
        }
    }
}

#[derive(Resource, Debug, Clone)]
pub struct CreateMarketRateInputData {
    pub seed: Seed,
    pub market_id: MarketId,
    pub market_rate_id: MarketId,
    pub rates: Vec<MarketRateInputData>,
    pub rates_metadata: String,
    pub loading: bool,
}

impl Default for CreateMarketRateInputData {
    fn default() -> Self {
        Self {
            seed: Seed::from("".to_string()),
            market_id: MarketId::from(0),
            market_rate_id: MarketId::from(0),
            rates: vec![MarketRateInputData::default()],
            rates_metadata: String::default(),
            loading: false,
        }
    }
}

pub fn create_market_rate_ui(ui: &mut egui::Ui, market: &mut ResMut<MarketUi>) {
    ui.label("Create Market Rate");
    ui.separator();
    ui.label("Seed");
    ui.text_edit_singleline(&mut *market.data.input.create_market_rate.seed);
    ui.label("Market ID");
    ui.add(egui::DragValue::new(&mut *market.data.input.create_market_rate.market_id).speed(1.0));
    ui.label("Market Rate ID");
    ui.add(
        egui::DragValue::new(&mut *market.data.input.create_market_rate.market_rate_id).speed(1.0),
    );
    ui.label("Rates");
    if ui.button("Add Rate").clicked() {
        market
            .data
            .input
            .create_market_rate
            .rates
            .push(MarketRateInputData::default());
    }
    let rates = market.data.input.create_market_rate.rates.clone();
    let mut rate_remove_index: Option<usize> = None;
    for (i, _) in rates.iter().enumerate() {
        ui.label(format!("Rate {}", i));
        ui.label("Class ID");
        ui.add(
            egui::DragValue::new::<u64>(
                &mut market.data.input.create_market_rate.rates[i].class_id,
            )
            .speed(0.1),
        );
        ui.label("Asset ID");
        ui.add(
            egui::DragValue::new::<u64>(
                &mut market.data.input.create_market_rate.rates[i].asset_id,
            )
            .speed(0.1),
        );
        market_rate_action_ui(ui, &mut market.data.input.create_market_rate.rates[i]);
        market_rate_account_ui(ui, &mut market.data.input.create_market_rate.rates[i]);
        if ui.button("Remove").clicked() {
            rate_remove_index = Some(i);
        }
        ui.separator();
    }
    if let Some(index) = rate_remove_index {
        market.data.input.create_market_rate.rates.remove(index);
    }
    ui.label("Rates Metadata");
    ui.text_edit_multiline(&mut market.data.input.create_market_rate.rates_metadata);
    ui.separator();
    if market.data.input.create_market_rate.loading {
        ui.separator();
        ui.add(egui::Spinner::default());
    } else {
        if ui.button("Create Market Rate").clicked() {
            market
                .channels
                .create_market_rate
                .input_tx
                .0
                .send(CreateMarketRateRequest {
                    input: CreateMarketRateInput {
                        seed: market.data.input.create_market_rate.seed.clone(),
                        market_id: market.data.input.create_market_rate.market_id,
                        market_rate_id: market.data.input.create_market_rate.market_rate_id,
                        rates: Rates {
                            rates: market
                                .data
                                .input
                                .create_market_rate
                                .rates
                                .iter()
                                .map(|rate| AssetRate {
                                    class_id: rate.class_id,
                                    asset_id: rate.asset_id,
                                    action: match rate.action_ui {
                                        MarketRateAction::Transfer => RateAction::Transfer(
                                            Amount::from(rate.action_data.transfer as i128),
                                        ),
                                        MarketRateAction::MarketTransfer => {
                                            RateAction::MarketTransfer(
                                                AMM::Constant,
                                                rate.action_data.market_transfer.class_id,
                                                rate.action_data.market_transfer.asset_id,
                                            )
                                        }
                                        MarketRateAction::Mint => RateAction::Mint(Amount::from(
                                            rate.action_data.mint as i128,
                                        )),
                                        MarketRateAction::Burn => RateAction::Burn(Amount::from(
                                            rate.action_data.burn as i128,
                                        )),
                                        MarketRateAction::Has => RateAction::Has(
                                            rate.action_data.has.amount_op.clone(),
                                            Amount::from(rate.action_data.has.amount as i128),
                                        ),
                                    },
                                    from: match rate.from.rate_account {
                                        MarketRateAccount::Buyer => RateAccount::Buyer,
                                        MarketRateAccount::Market => RateAccount::Market,
                                        MarketRateAccount::Account => {
                                            RateAccount::Account(rate.from.account.clone())
                                        }
                                    },
                                    to: match rate.to.rate_account {
                                        MarketRateAccount::Buyer => RateAccount::Buyer,
                                        MarketRateAccount::Market => RateAccount::Market,
                                        MarketRateAccount::Account => {
                                            RateAccount::Account(rate.to.account.clone())
                                        }
                                    },
                                })
                                .collect(),
                            metadata: serde_json::from_str(
                                &market.data.input.create_market_rate.rates_metadata,
                            )
                            .unwrap(),
                        },
                    },
                })
                .unwrap();
            market.data.input.create_market_rate.loading = true;
        }
    }
    if let Some(output) = &market.data.output.create_market_rate {
        ui.separator();
        ui.label("Market ID");
        ui.text_edit_singleline(&mut u64::from(output.market_id).to_string());
        ui.label("Market Rate ID");
        ui.text_edit_singleline(&mut u64::from(output.market_rate_id).to_string());
        ui.label("Who");
        ui.text_edit_singleline(&mut output.who.as_str());
    }
}

pub fn handle_create_market_rate_response(
    mut market: ResMut<MarketUi>,
    tokio_runtime: Res<TokioRuntime>,
) {
    if let Ok(created_result) = market.channels.create_market_rate.output_rx.0.try_recv() {
        if let Some(created) = created_result {
            market.data.output.create_market_rate = Some(created);
        }
        market.data.input.create_market_rate.loading = false;
    }

    request_handler::<CreateMarketRateRequest, CreateMarketRateInput, CreateMarketRateOutput>(
        tokio_runtime.runtime.clone(),
        market.channels.create_market_rate.input_rx.clone(),
        market.channels.create_market_rate.output_tx.clone(),
    );
}

pub struct CreateMarketRatePlugin;

impl Plugin for CreateMarketRatePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(handle_create_market_rate_response);
    }
}
