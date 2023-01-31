use bevy::prelude::*;
use bevy_egui::egui;
use sugarfunge_api_types::bag::*;

pub mod create;
pub mod deposit;
pub mod register;
pub mod sweep;

#[derive(Resource, Default)]
pub struct BagUi {
    pub actions: BagActions,
    pub data: BagData,
    pub channels: BagChannels,
}

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
    create: create::CreateBagInputData,
    register: register::RegisterBagInputData,
    sweep: sweep::SweepBagInputData,
    deposit: deposit::DepositBagInputData,
}

#[derive(Resource, Default, Debug)]
pub struct BagOutputData {
    create: Option<CreateOutput>,
    register: Option<RegisterOutput>,
    sweep: Option<SweepOutput>,
    deposit: Option<DepositOutput>,
}

#[derive(Resource, Default)]
pub struct BagData {
    input: BagInputData,
    output: BagOutputData,
}

#[derive(Resource, Default)]
pub struct BagChannels {
    create: create::CreateBagChannel,
    register: register::RegisterBagChannel,
    sweep: sweep::SweepBagChannel,
    deposit: deposit::DepositBagChannel,
}

pub fn bag_ui(ui: &mut egui::Ui, bag: &mut ResMut<BagUi>) {
    ui.horizontal(|ui| {
        ui.selectable_value(&mut bag.actions, BagActions::CreateBag, "Create");
        ui.selectable_value(&mut bag.actions, BagActions::RegisterBag, "Register");
        ui.selectable_value(&mut bag.actions, BagActions::SweepBag, "Sweep");
        ui.selectable_value(&mut bag.actions, BagActions::DepositBag, "Deposit");
    });
    ui.separator();
    match &bag.actions {
        BagActions::CreateBag => {
            create::create_bag_ui(ui, bag);
        }
        BagActions::RegisterBag => {
            register::register_bag_ui(ui, bag);
        }
        BagActions::SweepBag => {
            sweep::sweep_bag_ui(ui, bag);
        }
        BagActions::DepositBag => {
            deposit::deposit_bag_ui(ui, bag);
        }
    }
}

pub struct BagPlugin;

impl Plugin for BagPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<BagUi>()
            .add_plugin(create::CreateBagPlugin)
            .add_plugin(register::RegisterBagPlugin)
            .add_plugin(sweep::SweepBagPlugin)
            .add_plugin(deposit::DepositBagPlugin);
    }
}
