use bevy::prelude::*;
use bevy_egui::egui;
use sugarfunge_api_types::{
    market::{CreateMarketRateInput, CreateMarketRateOutput, RateAccount, Rates},
    primitives::{Amount, AssetId, ClassId, MarketId, Seed},
};

use crate::{prelude::*, util::*};

use self::{rate_account::*, rate_action::*};

use super::{MarketInputData, MarketOutputData};

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
    create_market_rate_tx: &Res<InputSender<CreateMarketRateRequest>>,
    market_output: &Res<MarketOutputData>,
) {
    ui.label("Create Market Rate");
    ui.separator();
    ui.label("Seed");
    ui.text_edit_singleline(&mut market_input.create_market_rate_input.seed);
    ui.label("Market ID");
    ui.add(egui::DragValue::new(&mut market_input.create_market_rate_input.market_id).speed(1.0));
    ui.label("Market Rate ID");
    ui.add(
        egui::DragValue::new(&mut market_input.create_market_rate_input.market_rate_id).speed(1.0),
    );
    ui.label("Rates");
    if ui.button("Add Rate").clicked() {
        market_input
            .create_market_rate_input
            .rates
            .push(MarketRateInputData::default());
    }
    let rates = market_input.create_market_rate_input.rates.clone();
    let mut rate_remove_index: Option<usize> = None;
    for (i, _) in rates.iter().enumerate() {
        ui.label(format!("Rate {}", i));
        ui.label("Class ID");
        ui.add(
            egui::DragValue::new::<u64>(
                &mut market_input.create_market_rate_input.rates[i].class_id,
            )
            .speed(0.1),
        );
        ui.label("Asset ID");
        ui.add(
            egui::DragValue::new::<u64>(
                &mut market_input.create_market_rate_input.rates[i].asset_id,
            )
            .speed(0.1),
        );
        market_rate_action_ui(ui, &mut market_input.create_market_rate_input.rates[i]);
        market_rate_account_ui(ui, &mut market_input.create_market_rate_input.rates[i]);
        if ui.button("Remove").clicked() {
            rate_remove_index = Some(i);
        }
        ui.separator();
    }
    if let Some(index) = rate_remove_index {
        market_input.create_market_rate_input.rates.remove(index);
    }
}

pub struct CreateMarketRatePlugin;

impl Plugin for CreateMarketRatePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_in_out_channels::<CreateMarketRateRequest, CreateMarketRateOutput>)
        .add_system(
            request_handler::<CreateMarketRateRequest, CreateMarketRateInput, CreateMarketRateOutput>,
        );
    }
}
