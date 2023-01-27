use bevy::prelude::*;
use bevy_egui::egui;
use sugarfunge_api_types::{
    bag::{DepositInput, DepositOutput},
    primitives::{Account, AssetId, Balance, Seed, ClassId},
};

use crate::{prelude::*, util::*};

use super::{BagInputData, BagOutputData};

#[derive(Debug)]
pub struct DepositBagRequest {
    pub input: DepositInput,
}

impl Request<DepositInput> for DepositBagRequest {
    fn endpoint(&self) -> &str {
        "bag/deposit"
    }

    fn input(&self) -> Option<DepositInput> {
        Some(DepositInput {
            seed: self.input.seed.clone(),
            bag: self.input.bag.clone(),
            class_ids: self.input.class_ids.clone(),
            asset_ids: self.input.asset_ids.clone(),
            amounts: self.input.amounts.clone(),
        })
    }
}

#[derive(Resource, Debug, Clone)]
pub struct DepositBagInputData {
    pub seed: Seed,
    pub bag: Account,
    pub class_ids: Vec<u64>,
    pub asset_ids: Vec<Vec<u64>>,
    pub amounts: Vec<Vec<u64>>,
    pub loading: bool,
}

impl Default for DepositBagInputData {
    fn default() -> Self {
        Self {
            seed: Seed::from("".to_string()),
            bag: Account::from("".to_string()),
            class_ids: vec![],
            asset_ids: vec![],
            amounts: vec![],
            loading: false,
        }
    }
}

pub fn deposit_bag_ui(
    ui: &mut egui::Ui,
    bag_input: &mut ResMut<BagInputData>,
    registered_tx: &Res<InputSender<DepositBagRequest>>,
    bag_output: &Res<BagOutputData>,
) {
    ui.label("Deposit Bag");
    ui.separator();
    ui.label("Seed");
    ui.text_edit_singleline(&mut *bag_input.deposit_input.seed);
    ui.label("Bag");
    ui.text_edit_singleline(&mut *bag_input.deposit_input.bag);
    ui.label("Class IDs");
    vec_u64_input_ui(ui, &mut bag_input.deposit_input.class_ids);
    ui.label("Asset IDs");
    vec_of_vec_u64_input_ui(ui, &mut bag_input.deposit_input.asset_ids, "Asset ID");
    ui.label("Amounts");
    ui.label("The Amounts are represented in 10^18 units.");
    vec_of_vec_u64_input_ui(ui, &mut bag_input.deposit_input.amounts, "Amount");
    if bag_input.deposit_input.loading {
        ui.separator();
        ui.add(egui::Spinner::default());
    } else {
        if ui.button("Deposit").clicked() {
            let class_ids: Vec<ClassId> =
                transform_vec_of_u64_to_class_id(bag_input.deposit_input.class_ids.clone());
            let asset_ids: Vec<Vec<AssetId>> =
                transform_doublevec_of_u64_to_asset_id(bag_input.deposit_input.asset_ids.clone());
            let amounts: Vec<Vec<Balance>> =
                transform_doublevec_of_u64_to_balance(bag_input.deposit_input.amounts.clone());
            registered_tx
                .send(DepositBagRequest {
                    input: DepositInput {
                        seed: bag_input.deposit_input.seed.clone(),
                        bag: bag_input.deposit_input.bag.clone(),
                        class_ids,
                        asset_ids,
                        amounts,
                    },
                })
                .unwrap();
            bag_input.deposit_input.loading = true;
        }
    }
    if let Some(output) = &bag_output.deposit_output {
        ui.separator();
        ui.label("Bag");
        ui.text_edit_singleline(&mut output.bag.as_str());
        ui.label("Who");
        ui.text_edit_singleline(&mut output.who.as_str());
    }
}

pub fn handle_deposit_response(
    mut bag_output: ResMut<BagOutputData>,
    mut bag_input: ResMut<BagInputData>,
    deposited_rx: Res<OutputReceiver<DepositOutput>>,
) {
    if let Ok(deposited_result) = deposited_rx.0.try_recv() {
        if let Some(deposited) = deposited_result {
            bag_output.deposit_output = Some(deposited);
        }
        bag_input.deposit_input.loading = false;
    }
}

pub struct DepositBagPlugin;

impl Plugin for DepositBagPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_in_out_channels::<DepositBagRequest, DepositOutput>)
            .add_system(request_handler::<DepositBagRequest, DepositInput, DepositOutput>)
            .add_system(handle_deposit_response);
    }
}
