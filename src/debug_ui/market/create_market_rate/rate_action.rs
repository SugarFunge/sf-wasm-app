use bevy::prelude::*;
use bevy_egui::egui;
use sugarfunge_api_types::{
    market::AmountOp,
    primitives::{AssetId, ClassId},
};

use super::MarketRateInputData;

#[derive(Resource, Debug, Clone)]
pub struct MarketHasAction {
    pub amount_op: AmountOp,
    pub amount: u64,
}

impl Default for MarketHasAction {
    fn default() -> Self {
        Self {
            amount_op: AmountOp::Equal,
            amount: 0,
        }
    }
}

#[derive(Resource, Debug, Clone)]
pub struct MarketTransferAction {
    pub class_id: ClassId,
    pub asset_id: AssetId,
}

impl Default for MarketTransferAction {
    fn default() -> Self {
        Self {
            class_id: ClassId::from(0),
            asset_id: AssetId::from(0),
        }
    }
}

#[derive(Resource, Debug, Default, Clone)]
pub struct MarketRateActionInputData {
    pub transfer: u64,
    pub market_transfer: MarketTransferAction,
    pub mint: u64,
    pub burn: u64,
    pub has: MarketHasAction,
}

#[derive(Resource, Debug, Clone, PartialEq)]
pub enum MarketRateAction {
    Transfer,
    MarketTransfer,
    Mint,
    Burn,
    Has,
}

pub fn market_rate_action_ui(ui: &mut egui::Ui, rate: &mut MarketRateInputData) {
    ui.label("Action");
    ui.horizontal(|ui| {
        ui.selectable_value(&mut rate.action_ui, MarketRateAction::Transfer, "Transfer");
        ui.selectable_value(
            &mut rate.action_ui,
            MarketRateAction::MarketTransfer,
            "Market Transfer",
        );
        ui.selectable_value(&mut rate.action_ui, MarketRateAction::Mint, "Mint");
        ui.selectable_value(&mut rate.action_ui, MarketRateAction::Burn, "Burn");
        ui.selectable_value(&mut rate.action_ui, MarketRateAction::Has, "Has");
    });
    ui.separator();
    match &rate.action_ui {
        MarketRateAction::Transfer => {
            ui.label("Amount");
            ui.add(egui::DragValue::new(&mut rate.action_data.transfer).speed(1.0));
        }
        MarketRateAction::MarketTransfer => {
            ui.label("Market Transfer Class ID");
            ui.add(
                egui::DragValue::new(&mut *rate.action_data.market_transfer.class_id).speed(1.0),
            );
            ui.label("Market Transfer Asset ID");
            ui.add(
                egui::DragValue::new(&mut *rate.action_data.market_transfer.asset_id).speed(1.0),
            );
        }
        MarketRateAction::Mint => {
            ui.label("Amount");
            ui.add(egui::DragValue::new(&mut rate.action_data.mint).speed(1.0));
        }
        MarketRateAction::Burn => {
            ui.label("Amount");
            ui.add(egui::DragValue::new(&mut rate.action_data.burn).speed(1.0));
        }
        MarketRateAction::Has => {
            ui.label("Amount Option");
            ui.horizontal(|ui| {
                ui.selectable_value(
                    &mut rate.action_data.has.amount_op,
                    AmountOp::Equal,
                    "Equal",
                );
                ui.selectable_value(
                    &mut rate.action_data.has.amount_op,
                    AmountOp::LessThan,
                    "Less Than",
                );
                ui.selectable_value(
                    &mut rate.action_data.has.amount_op,
                    AmountOp::LessEqualThan,
                    "Less Equal Than",
                );
            });
            ui.horizontal(|ui| {
                ui.selectable_value(
                    &mut rate.action_data.has.amount_op,
                    AmountOp::GreaterThan,
                    "Greater Than",
                );
                ui.selectable_value(
                    &mut rate.action_data.has.amount_op,
                    AmountOp::GreaterEqualThan,
                    "Greater Equal Than",
                );
            });
            ui.label("Amount");
            ui.add(egui::DragValue::new(&mut rate.action_data.has.amount).speed(1.0));
        }
    }
}
