use bevy::prelude::*;
use bevy_egui::egui;
use sugarfunge_api_types::market::*;

pub mod create_market;
pub mod create_market_rate;
pub mod deposit_market_assets;
pub mod exchange_market_assets;

#[derive(Resource, Default)]
pub struct MarketUi {
    pub actions: MarketActions,
    pub data: MarketData,
    pub channels: MarketChannels,
}

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
    create_market: create_market::CreateMarketInputData,
    create_market_rate: create_market_rate::CreateMarketRateInputData,
    deposit_market_assets: deposit_market_assets::DepositMarketAssetsInputData,
    exchange_market_assets: exchange_market_assets::ExchangeMarketAssetsInputData,
}

#[derive(Resource, Default, Debug)]
pub struct MarketOutputData {
    create_market: Option<CreateMarketOutput>,
    create_market_rate: Option<CreateMarketRateOutput>,
    deposit_market_assets: Option<DepositAssetsOutput>,
    exchange_market_assets: Option<ExchangeAssetsOutput>,
}

#[derive(Resource, Default)]
pub struct MarketData {
    input: MarketInputData,
    output: MarketOutputData,
}

#[derive(Resource, Default)]
pub struct MarketChannels {
    create_market: create_market::CreateMarketChannel,
    create_market_rate: create_market_rate::CreateMarketRateChannel,
    deposit_market_assets: deposit_market_assets::DepositMarketAssetsChannel,
    exchange_market_assets: exchange_market_assets::ExchangeMarketAssetsChannel,
}

pub fn market_ui(ui: &mut egui::Ui, market: &mut ResMut<MarketUi>) {
    ui.horizontal(|ui| {
        ui.selectable_value(&mut market.actions, MarketActions::CreateMarket, "Create");
        ui.selectable_value(
            &mut market.actions,
            MarketActions::CreateMarketRate,
            "Create Rate",
        );
        ui.selectable_value(
            &mut market.actions,
            MarketActions::DepositMarketAssets,
            "Deposit",
        );
        ui.selectable_value(
            &mut market.actions,
            MarketActions::ExchangeMarketAssets,
            "Exchange",
        );
    });
    ui.separator();
    match &market.actions {
        MarketActions::CreateMarket => {
            create_market::create_market_ui(ui, market);
        }
        MarketActions::CreateMarketRate => {
            create_market_rate::create_market_rate_ui(ui, market);
        }
        MarketActions::DepositMarketAssets => {
            deposit_market_assets::deposit_market_assets_ui(ui, market);
        }
        MarketActions::ExchangeMarketAssets => {
            exchange_market_assets::exchange_market_assets_ui(ui, market);
        }
    }
}

pub struct MarketPlugin;

impl Plugin for MarketPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MarketUi>()
            .add_plugin(create_market::CreateMarketPlugin)
            .add_plugin(create_market_rate::CreateMarketRatePlugin)
            .add_plugin(deposit_market_assets::DepositMarketAssetsPlugin)
            .add_plugin(exchange_market_assets::ExchangeMarketAssetsPlugin);
    }
}
