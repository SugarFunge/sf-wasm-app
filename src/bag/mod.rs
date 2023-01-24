use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};
use sugarfunge_api_types::bag::*;

use crate::prelude::*;

pub mod create;
pub mod deposit;
pub mod register;
pub mod sweep;

#[derive(Resource, Debug, Default, Eq, PartialEq)]
pub enum BagActions {
    #[default]
    CreateBag,
    RegisterBag,
    SweepBag,
    DepositBag,
}

#[derive(Resource, Debug, Default, Clone)]
pub struct BagInputData {
    create_input: create::CreateBagInputData,
    register_input: register::RegisterBagInputData,
    sweep_input: sweep::SweepBagInputData,
    deposit_input: deposit::DepositBagInputData,
}

#[derive(Resource, Default, Debug)]
pub struct BagOutputData {
    create_output: Option<CreateOutput>,
    register_output: Option<RegisterOutput>,
    sweep_output: Option<SweepOutput>,
    deposit_output: Option<DepositOutput>,
}

pub fn bag_ui(
    mut ctx: ResMut<EguiContext>,
    mut bag_actions: ResMut<BagActions>,
    mut bag_input: ResMut<BagInputData>,
    bag_output: Res<BagOutputData>,
    create_tx: Res<InputSender<create::CreateBagRequest>>,
    register_tx: Res<InputSender<register::RegisterBagRequest>>,
    sweep_tx: Res<InputSender<sweep::SweepBagRequest>>,
    deposit_tx: Res<InputSender<deposit::DepositBagRequest>>,
) {
    egui::Window::new("Bag")
        .scroll2([false, true])
        .show(ctx.ctx_mut(), |ui| {
            ui.horizontal(|ui| {
                ui.selectable_value(&mut *bag_actions, BagActions::CreateBag, "Create");
                ui.selectable_value(&mut *bag_actions, BagActions::RegisterBag, "Register");
                ui.selectable_value(&mut *bag_actions, BagActions::SweepBag, "Sweep");
                ui.selectable_value(&mut *bag_actions, BagActions::DepositBag, "Deposit");
            });
            ui.separator();
            match &*bag_actions {
                BagActions::CreateBag => {
                    create::create_bag_ui(ui, &mut bag_input, &create_tx, &bag_output);
                }
                BagActions::RegisterBag => {
                    register::register_bag_ui(ui, &mut bag_input, &register_tx, &bag_output);
                }
                BagActions::SweepBag => {
                    sweep::sweep_bag_ui(ui, &mut bag_input, &sweep_tx, &bag_output);
                }
                BagActions::DepositBag => {
                    deposit::deposit_bag_ui(ui, &mut bag_input, &deposit_tx, &bag_output);
                }
            }
        });
}

pub struct BagPlugin;

impl Plugin for BagPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<BagActions>()
            .init_resource::<BagInputData>()
            .init_resource::<BagOutputData>()
            .add_system(bag_ui)
            .add_plugin(create::CreateBagPlugin)
            .add_plugin(register::RegisterBagPlugin)
            .add_plugin(sweep::SweepBagPlugin)
            .add_plugin(deposit::DepositBagPlugin);
    }
}
