use bevy::prelude::*;
use bevy_egui::egui;
use sugarfunge_api_types::{
    market::{CreateMarketRateInput, CreateMarketRateOutput, Rates, RateAction, RateAccount},
    primitives::{MarketId, Seed, Amount},
};

use crate::{prelude::*, util::*};

use super::{MarketInputData, MarketOutputData};

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

#[derive(Resource, Debug, Clone)]
pub struct MarketRateInputData {
    pub class_id: u64,
    pub asset_id: u64,
    pub action: RateAction,
    pub from: RateAccount,
    pub to: RateAccount,
}

impl Default for MarketRateInputData {
    fn default() -> Self {
        Self {
            class_id: u64::default(),
            asset_id: u64::default(),
            action: RateAction::Transfer(Amount::from(1)),
            from: RateAccount::Market,
            to: RateAccount::Buyer,
        }
    }
}

#[derive(Resource, Debug, Default, Clone)]
pub struct CreateMarketRateInputData {
    pub seed: String,
    pub market_id: u64,
    pub market_rate_id: u64,
    pub rates: Vec<MarketRateInputData>,
    pub loading: bool,
}

pub fn create_market_rate_ui(
    ui: &mut egui::Ui,
    market_input: &mut ResMut<MarketInputData>,
    market_output: &Res<MarketOutputData>,
    create_market_rate_tx: &Res<InputSender<CreateMarketRateRequest>>,
) {
    ui.label("Create Market Rate");
    ui.separator();
    ui.label("Seed");
    ui.text_edit_singleline(&mut market_input.create_market_rate_input.seed);
    ui.label("Market ID");
    ui.add(egui::DragValue::new(&mut market_input.create_market_rate_input.market_id).speed(1.0));
    ui.label("Market Rate ID");
    ui.add(egui::DragValue::new(&mut market_input.create_market_rate_input.market_rate_id).speed(1.0));
    ui.separator();
    ui.label("Rates");
}