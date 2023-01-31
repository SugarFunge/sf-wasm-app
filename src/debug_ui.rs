use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};

use crate::{
    account::{account_ui, AccountUi, AccountPlugin},
    asset::{asset_ui, AssetUi, AssetPlugin},
    bag::{bag_ui, BagUi, BagPlugin},
    bundle::{bundle_ui, BundleUi, BundlePlugin},
    class::{class_ui, ClassUi, ClassPlugin},
    market::{market_ui, MarketUi, MarketPlugin},
    validator::{validator_ui, ValidatorUi, ValidatorPlugin},
};

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
    mut ctx: ResMut<EguiContext>,
    mut debug_actions: ResMut<DebugUiActions>,
    mut asset: ResMut<AssetUi>,
    mut account: ResMut<AccountUi>,
    mut class: ResMut<ClassUi>,
    mut bag: ResMut<BagUi>,
    mut bundle: ResMut<BundleUi>,
    mut market: ResMut<MarketUi>,
    mut validator: ResMut<ValidatorUi>,
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
                DebugUiActions::Account => account_ui(ui, &mut account),
                DebugUiActions::Class => class_ui(ui, &mut class),
                DebugUiActions::Asset => asset_ui(ui, &mut asset),
                DebugUiActions::Bag => bag_ui(ui, &mut bag),
                DebugUiActions::Bundle => bundle_ui(ui, &mut bundle),
                DebugUiActions::Market => market_ui(ui, &mut market),
                DebugUiActions::Validator => validator_ui(ui, &mut validator),
            }
        });
}

pub struct DebugUiPlugin;

impl Plugin for DebugUiPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<DebugUiActions>()
            .add_plugin(AccountPlugin)
            .add_plugin(ClassPlugin)
            .add_plugin(AssetPlugin)
            .add_plugin(BagPlugin)
            .add_plugin(BundlePlugin)
            .add_plugin(MarketPlugin)
            .add_plugin(ValidatorPlugin)
            .add_system(debug_ui);
    }
}
