use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

pub mod account;
pub mod asset;
pub mod bag;
pub mod bundle;
pub mod class;
pub mod market;
pub mod validator;

#[derive(Resource, Debug, Default, Eq, PartialEq)]
pub enum DebugUiActions {
    #[default]
    Account,
    Class,
    Asset,
    Bag,
    Bundle,
    Market,
    Validator,
}

pub fn debug_ui(
    mut ctx: EguiContexts,
    mut debug_actions: ResMut<DebugUiActions>,
    mut asset: ResMut<asset::AssetUi>,
    mut account: ResMut<account::AccountUi>,
    mut class: ResMut<class::ClassUi>,
    mut bag: ResMut<bag::BagUi>,
    mut bundle: ResMut<bundle::BundleUi>,
    mut market: ResMut<market::MarketUi>,
    mut validator: ResMut<validator::ValidatorUi>,
) {
    egui::Window::new("SugarFunge Debug UI")
        .scroll2([false, true])
        .show(&mut ctx.ctx_mut(), |ui| {
            ui.horizontal(|ui| {
                ui.selectable_value(&mut *debug_actions, DebugUiActions::Account, "Account");
                ui.selectable_value(&mut *debug_actions, DebugUiActions::Class, "Class");
                ui.selectable_value(&mut *debug_actions, DebugUiActions::Asset, "Asset");
                ui.selectable_value(&mut *debug_actions, DebugUiActions::Bag, "Bag");
                ui.selectable_value(&mut *debug_actions, DebugUiActions::Bundle, "Bundle");
                ui.selectable_value(&mut *debug_actions, DebugUiActions::Market, "Market");
                ui.selectable_value(&mut *debug_actions, DebugUiActions::Validator, "Validator");
            });
            ui.separator();
            match &*debug_actions {
                DebugUiActions::Account => account::account_ui(ui, &mut account),
                DebugUiActions::Class => class::class_ui(ui, &mut class),
                DebugUiActions::Asset => asset::asset_ui(ui, &mut asset),
                DebugUiActions::Bag => bag::bag_ui(ui, &mut bag),
                DebugUiActions::Bundle => bundle::bundle_ui(ui, &mut bundle),
                DebugUiActions::Market => market::market_ui(ui, &mut market),
                DebugUiActions::Validator => validator::validator_ui(ui, &mut validator),
            }
        });
}

pub struct DebugUiPlugin;

impl Plugin for DebugUiPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<DebugUiActions>()
            .add_plugin(account::AccountPlugin)
            .add_plugin(class::ClassPlugin)
            .add_plugin(asset::AssetPlugin)
            .add_plugin(bag::BagPlugin)
            .add_plugin(bundle::BundlePlugin)
            .add_plugin(market::MarketPlugin)
            .add_plugin(validator::ValidatorPlugin)
            .add_system(debug_ui);
    }
}
