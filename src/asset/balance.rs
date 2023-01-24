use bevy::prelude::*;
use bevy_egui::egui;
use sugarfunge_api_types::{
    asset::{AssetBalanceInput, AssetBalanceOutput},
    primitives::{Account, AssetId, ClassId},
};

use crate::{
    prelude::*,
    util::{request_handler, setup_in_out_channels},
};

use super::{AssetInputData, AssetOutputData};

#[derive(Debug)]
pub struct AssetBalanceRequest {
    pub input: AssetBalanceInput,
}

impl Request<AssetBalanceInput> for AssetBalanceRequest {
    fn endpoint(&self) -> &str {
        "asset/balance"
    }

    fn input(&self) -> Option<AssetBalanceInput> {
        Some(AssetBalanceInput {
            class_id: self.input.class_id.clone(),
            asset_id: self.input.asset_id.clone(),
            account: self.input.account.clone(),
        })
    }
}

#[derive(Resource, Debug, Default, Clone)]
pub struct AssetBalanceInputData {
    pub class_id: u64,
    pub asset_id: u64,
    pub account: String,
    pub loading: bool,
}

pub fn asset_balance_ui(
    ui: &mut egui::Ui,
    asset_input: &mut ResMut<AssetInputData>,
    balance_tx: &Res<InputSender<AssetBalanceRequest>>,
    asset_output: &Res<AssetOutputData>,
) {
    ui.label("Asset Balance");
    ui.separator();
    ui.label("Class ID");
    ui.add(egui::DragValue::new::<u64>(&mut asset_input.balance_input.class_id).speed(0.1));
    ui.label("Asset ID");
    ui.add(egui::DragValue::new::<u64>(&mut asset_input.balance_input.asset_id).speed(0.1));
    ui.label("Account");
    ui.text_edit_singleline(&mut asset_input.balance_input.account);
    ui.separator();
    if asset_input.balance_input.loading {
        ui.separator();
        ui.add(egui::Spinner::default());
    } else {
        if ui.button("Get Balance").clicked() {
            balance_tx
                .send(AssetBalanceRequest {
                    input: AssetBalanceInput {
                        class_id: ClassId::from(asset_input.balance_input.class_id.clone()),
                        asset_id: AssetId::from(asset_input.balance_input.asset_id.clone()),
                        account: Account::from(asset_input.balance_input.account.clone()),
                    },
                })
                .unwrap();
        }
    }
    if let Some(output) = &asset_output.balance_output {
        ui.separator();
        ui.label("Amount");
        ui.text_edit_singleline(&mut u128::from(output.amount).to_string());
    }
}

pub fn handle_balance_response(
    mut asset_output: ResMut<AssetOutputData>,
    mut asset_input: ResMut<AssetInputData>,
    balance_rx: Res<OutputReceiver<AssetBalanceOutput>>,
) {
    if let Ok(balance_result) = balance_rx.0.try_recv() {
        if let Some(balance) = balance_result {
            asset_output.balance_output = Some(balance);
        }
        asset_input.balance_input.loading = false;
    }
}

pub struct AssetBalancePlugin;

impl Plugin for AssetBalancePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_in_out_channels::<AssetBalanceRequest, AssetBalanceOutput>)
            .add_system(
                request_handler::<AssetBalanceRequest, AssetBalanceInput, AssetBalanceOutput>,
            )
            .add_system(handle_balance_response);
    }
}
