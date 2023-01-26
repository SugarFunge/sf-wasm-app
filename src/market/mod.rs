use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};
use sugarfunge_api_types::market::*;

use crate::prelude::*;

pub mod create_market;
pub mod create_market_rate;

#[derive(Resource, Debug, Default, Eq, PartialEq)]
pub enum MarketActions {
    #[default]
    CreateMarket,
    CreateMarketRate,
    DepositMarketAssets,
    ExchangeMarketAssets,
}

#[derive(Resource, Debug, Default, Clone)]
pub struct MarketInputData {
    create_market_input: create_market::CreateMarketInputData,
    create_market_rate_input: create_market_rate::CreateMarketRateInputData,
    // deposit_market_assets_input: deposit_market_assets::DepositMarketAssetsInputData,
    // exchange_market_assets_input: exchange_market_assets::ExchangeMarketAssetsInputData,
}

#[derive(Resource, Default, Debug)]
pub struct MarketOutputData {
    create_market_output: Option<CreateMarketOutput>,
    create_market_rate_output: Option<CreateMarketRateOutput>,
    deposit_market_assets_output: Option<DepositAssetsOutput>,
    exchange_market_assets_output: Option<ExchangeAssetsOutput>,
}

pub fn market_ui(
    mut ctx: ResMut<EguiContext>,
    mut market_actions: ResMut<MarketActions>,
    mut market_input: ResMut<MarketInputData>,
    market_output: Res<MarketOutputData>,
    create_market_tx: Res<InputSender<create_market::CreateMarketRequest>>,
    // create_market_rate_tx: Res<InputSender<create_market_rate::CreateMarketRateRequest>>,
    // deposit_market_assets_tx: Res<InputSender<deposit_market_assets::DepositAssetsRequest>>,
    // exchange_market_assets_tx: Res<InputSender<exchange_market_assets::ExchangeAssetsRequest>>,
) {
    egui::Window::new("Market").show(&mut ctx.ctx_mut(), |ui| {
        ui.horizontal(|ui| {
            ui.selectable_value(&mut *market_actions, MarketActions::CreateMarket, "Create");
            ui.selectable_value(
                &mut *market_actions,
                MarketActions::CreateMarketRate,
                "Create Rate",
            );
            ui.selectable_value(
                &mut *market_actions,
                MarketActions::DepositMarketAssets,
                "Deposit",
            );
            ui.selectable_value(
                &mut *market_actions,
                MarketActions::ExchangeMarketAssets,
                "Exchange",
            );
        });
        ui.separator();
        match &*market_actions {
            MarketActions::CreateMarket => {
                create_market::create_market_ui(
                    ui,
                    &mut market_input,
                    &create_market_tx,
                    &market_output,
                );
            }
            MarketActions::CreateMarketRate => {
                // create_market_rate::create_market_rate_ui(
                //     ui,
                //     &mut market_input,
                //     &create_market_rate_tx,
                //     &market_output,
                // );
            }
            MarketActions::DepositMarketAssets => {
                // deposit_market_assets::deposit_market_assets_ui(
                //     ui,
                //     &mut market_input,
                //     &deposit_market_assets_tx,
                //     &market_output,
                // );
            }
            MarketActions::ExchangeMarketAssets => {
                // exchange_market_assets::exchange_market_assets_ui(
                //     ui,
                //     &mut market_input,
                //     &exchange_market_assets_tx,
                //     &market_output,
                // );
            }
        }
    });
}

pub struct MarketPlugin;

impl Plugin for MarketPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MarketActions>()
            .init_resource::<MarketInputData>()
            .init_resource::<MarketOutputData>()
            .add_plugin(create_market::CreateMarketPlugin)
            .add_system(market_ui);
    }
}
