use bevy::prelude::*;
use bevy_egui::egui;
use crossbeam::channel;
use sugarfunge_api_types::{
    bag::{DepositInput, DepositOutput},
    primitives::{Account, AssetId, Balance, ClassId, Seed},
};

use crate::{prelude::*, util::*};

use super::BagUi;

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

#[derive(Resource)]
pub struct DepositBagChannel {
    pub input_tx: InputSender<DepositBagRequest>,
    pub input_rx: InputReceiver<DepositBagRequest>,
    pub output_tx: OutputSender<DepositOutput>,
    pub output_rx: OutputReceiver<DepositOutput>,
}

impl Default for DepositBagChannel {
    fn default() -> Self {
        let (input_tx, input_rx) = channel::unbounded::<DepositBagRequest>();
        let (output_tx, output_rx) = channel::unbounded::<Option<DepositOutput>>();
        Self {
            input_tx: InputSender(input_tx),
            input_rx: InputReceiver(input_rx),
            output_tx: OutputSender(output_tx),
            output_rx: OutputReceiver(output_rx),
        }
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

pub fn deposit_bag_ui(ui: &mut egui::Ui, bag: &mut ResMut<BagUi>) {
    ui.label("Deposit Bag");
    ui.separator();
    ui.label("Seed");
    ui.text_edit_singleline(&mut *bag.data.input.deposit.seed);
    ui.label("Bag");
    ui.text_edit_singleline(&mut *bag.data.input.deposit.bag);
    ui.label("Class IDs");
    vec_u64_input_ui(ui, &mut bag.data.input.deposit.class_ids);
    ui.label("Asset IDs");
    vec_of_vec_u64_input_ui(ui, &mut bag.data.input.deposit.asset_ids, "Asset ID");
    ui.label("Amounts");
    vec_of_vec_u64_input_ui(ui, &mut bag.data.input.deposit.amounts, "Amount");
    if bag.data.input.deposit.loading {
        ui.separator();
        ui.add(egui::Spinner::default());
    } else {
        if ui.button("Deposit").clicked() {
            let class_ids: Vec<ClassId> =
                transform_vec_of_u64_to_class_id(bag.data.input.deposit.class_ids.clone());
            let asset_ids: Vec<Vec<AssetId>> =
                transform_doublevec_of_u64_to_asset_id(bag.data.input.deposit.asset_ids.clone());
            let amounts: Vec<Vec<Balance>> =
                transform_doublevec_of_u64_to_balance(bag.data.input.deposit.amounts.clone());
            bag.channels
                .deposit
                .input_tx
                .0
                .send(DepositBagRequest {
                    input: DepositInput {
                        seed: bag.data.input.deposit.seed.clone(),
                        bag: bag.data.input.deposit.bag.clone(),
                        class_ids,
                        asset_ids,
                        amounts,
                    },
                })
                .unwrap();
            bag.data.input.deposit.loading = true;
        }
    }
    if let Some(output) = &bag.data.output.deposit {
        ui.separator();
        ui.label("Bag");
        ui.text_edit_singleline(&mut output.bag.as_str());
        ui.label("Who");
        ui.text_edit_singleline(&mut output.who.as_str());
    }
}

pub fn handle_deposit_response(mut bag: ResMut<BagUi>, tokio_runtime: Res<TokioRuntime>) {
    if let Ok(deposited_result) = bag.channels.deposit.output_rx.0.try_recv() {
        if let Some(deposited) = deposited_result {
            bag.data.output.deposit = Some(deposited);
        }
        bag.data.input.deposit.loading = false;
    }

    request_handler::<DepositBagRequest, DepositInput, DepositOutput>(
        tokio_runtime.runtime.clone(),
        bag.channels.deposit.input_rx.clone(),
        bag.channels.deposit.output_tx.clone(),
    );
}

pub struct DepositBagPlugin;

impl Plugin for DepositBagPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(handle_deposit_response);
    }
}
