use bevy::prelude::*;
use bevy_egui::egui;
use sugarfunge_api_types::bundle::*;

pub mod burn;
pub mod mint;
pub mod register;

#[derive(Resource, Default)]
pub struct BundleUi {
    pub actions: BundleActions,
    pub data: BundleData,
    pub channels: BundleChannels,
}

#[derive(Resource, Debug, Default, Eq, PartialEq)]
pub enum BundleActions {
    #[default]
    RegisterBundle,
    MintBundle,
    BurnBundle,
}

#[derive(Resource, Debug, Default, Clone)]
pub struct BundleInputData {
    register: register::RegisterBundleInputData,
    mint: mint::MintBundleInputData,
    burn: burn::BurnBundleInputData,
}

#[derive(Resource, Default, Debug)]
pub struct BundleOutputData {
    register: Option<RegisterBundleOutput>,
    mint: Option<MintBundleOutput>,
    burn: Option<BurnBundleOutput>,
}

#[derive(Resource, Default)]
pub struct BundleData {
    input: BundleInputData,
    output: BundleOutputData,
}

#[derive(Resource, Default)]
pub struct BundleChannels {
    register: register::RegisterBundleChannel,
    mint: mint::MintBundleChannel,
    burn: burn::BurnBundleChannel,
}

pub fn bundle_ui(ui: &mut egui::Ui, bundle: &mut ResMut<BundleUi>) {
    ui.horizontal(|ui| {
        ui.selectable_value(
            &mut bundle.actions,
            BundleActions::RegisterBundle,
            "Register",
        );
        ui.selectable_value(&mut bundle.actions, BundleActions::MintBundle, "Mint");
        ui.selectable_value(&mut bundle.actions, BundleActions::BurnBundle, "Burn");
    });
    ui.separator();
    match &bundle.actions {
        BundleActions::RegisterBundle => {
            register::register_bundle_ui(ui, bundle);
        }
        BundleActions::MintBundle => {
            mint::mint_bundle_ui(ui, bundle);
        }
        BundleActions::BurnBundle => {
            burn::burn_bundle_ui(ui, bundle);
        }
    }
}

pub struct BundlePlugin;

impl Plugin for BundlePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<BundleUi>()
            .add_plugin(register::RegisterBundlePlugin)
            .add_plugin(mint::MintBundlePlugin)
            .add_plugin(burn::BurnBundlePlugin);
    }
}
