use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};
use sugarfunge_api_types::bundle::*;

use crate::prelude::*;

pub mod register;

#[derive(Resource, Debug, Default, Eq, PartialEq)]
pub enum BundleActions {
    #[default]
    RegisterBundle,
    MintBundle,
    BurnBundle,
}

#[derive(Resource, Debug, Default, Clone)]
pub struct BundleInputData {
    register_input: register::RegisterBundleInputData,
    // mint_input: mint::MintBundleInputData,
    // burn_input: burn::BurnBundleInputData,
}

#[derive(Resource, Default, Debug)]
pub struct BundleOutputData {
    register_output: Option<RegisterBundleOutput>,
    mint_output: Option<MintBundleOutput>,
    burn_output: Option<BurnBundleOutput>,
}

pub fn bundle_ui(
    mut ctx: ResMut<EguiContext>,
    mut bundle_actions: ResMut<BundleActions>,
    mut bundle_input: ResMut<BundleInputData>,
    bundle_output: Res<BundleOutputData>,
    register_tx: Res<InputSender<register::RegisterBundleRequest>>,
    // mint_tx: Res<InputSender<mint::MintBundleRequest>>,
    // burn_tx: Res<InputSender<burn::BurnBundleRequest>>,
) {
    egui::Window::new("Bundle").show(&mut ctx.ctx_mut(), |ui| {
        ui.horizontal(|ui| {
            ui.selectable_value(&mut *bundle_actions, BundleActions::RegisterBundle, "Register");
            ui.selectable_value(&mut *bundle_actions, BundleActions::MintBundle, "Mint");
            ui.selectable_value(&mut *bundle_actions, BundleActions::BurnBundle, "Burn");
        });
        ui.separator();
        match &*bundle_actions {
            BundleActions::RegisterBundle => {
                register::register_bundle_ui(ui, &mut bundle_input, &register_tx, &bundle_output);
            }
            BundleActions::MintBundle => {
                // mint::mint_bundle_ui(ui, &mut bundle_input, &mint_tx, &bundle_output);
            }
            BundleActions::BurnBundle => {
                // burn::burn_bundle_ui(ui, &mut bundle_input, &burn_tx, &bundle_output);
            }
        }
    });
}

pub struct BundlePlugin;

impl Plugin for BundlePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<BundleActions>()
            .init_resource::<BundleInputData>()
            .init_resource::<BundleOutputData>()
            .add_plugin(register::RegisterBundlePlugin)
            .add_system(bundle_ui);
    }
}