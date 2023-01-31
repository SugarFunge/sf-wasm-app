use bevy::prelude::*;
use bevy_egui::egui;
use sugarfunge_api_types::primitives::Account;

use super::MarketRateInputData;

#[derive(Resource, Debug, Default, Clone, PartialEq)]
pub enum MarketRateAccount {
    #[default]
    Market,
    Account,
    Buyer,
}

#[derive(Resource, Debug, Clone)]
pub struct MarketRateAccountInputData {
    pub account: Account,
    pub account_enabled: bool,
    pub rate_account: MarketRateAccount,
}

impl Default for MarketRateAccountInputData {
    fn default() -> Self {
        Self {
            account: Account::from("".to_string()),
            account_enabled: false,
            rate_account: MarketRateAccount::default(),
        }
    }
}

pub fn market_rate_account_ui(ui: &mut egui::Ui, rate: &mut MarketRateInputData) {
    ui.horizontal(|ui| {
        ui.label("From:");
        ui.radio_value(
            &mut rate.from.rate_account,
            MarketRateAccount::Market,
            "Market",
        );
        ui.radio_value(
            &mut rate.from.rate_account,
            MarketRateAccount::Account,
            "Account",
        );
        ui.radio_value(
            &mut rate.from.rate_account,
            MarketRateAccount::Buyer,
            "Buyer",
        );
    });
    if rate.from.rate_account == MarketRateAccount::Account {
        ui.horizontal(|ui| {
            ui.label("Account:");
            ui.text_edit_singleline(&mut *rate.from.account);
            rate.from.account_enabled = true;
        });
    } else {
        rate.from.account_enabled = false;
    }
    ui.horizontal(|ui| {
        ui.label("To:");
        ui.radio_value(
            &mut rate.to.rate_account,
            MarketRateAccount::Market,
            "Market",
        );
        ui.radio_value(
            &mut rate.to.rate_account,
            MarketRateAccount::Account,
            "Account",
        );
        ui.radio_value(&mut rate.to.rate_account, MarketRateAccount::Buyer, "Buyer");
    });
    if rate.to.rate_account == MarketRateAccount::Account {
        ui.horizontal(|ui| {
            ui.label("Account:");
            ui.text_edit_singleline(&mut *rate.to.account);
            rate.to.account_enabled = true;
        });
    } else {
        rate.to.account_enabled = false;
    }
}
