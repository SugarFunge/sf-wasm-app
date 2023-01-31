use bevy::prelude::*;
use bevy_egui::egui;
use crossbeam::channel;
use sugarfunge_api_types::{
    asset::{AssetBalanceInput, AssetBalanceOutput},
    primitives::{Account, AssetId, ClassId},
};

use crate::{prelude::*, util::request_handler};

use super::AssetUi;

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

#[derive(Resource)]
pub struct AssetBalanceChannel {
    pub input_tx: InputSender<AssetBalanceRequest>,
    pub input_rx: InputReceiver<AssetBalanceRequest>,
    pub output_tx: OutputSender<AssetBalanceOutput>,
    pub output_rx: OutputReceiver<AssetBalanceOutput>,
}

impl Default for AssetBalanceChannel {
    fn default() -> Self {
        let (input_tx, input_rx) = channel::unbounded::<AssetBalanceRequest>();
        let (output_tx, output_rx) = channel::unbounded::<Option<AssetBalanceOutput>>();
        Self {
            input_tx: InputSender(input_tx),
            input_rx: InputReceiver(input_rx),
            output_tx: OutputSender(output_tx),
            output_rx: OutputReceiver(output_rx),
        }
    }
}

#[derive(Resource, Debug, Clone)]
pub struct AssetBalanceInputData {
    pub class_id: ClassId,
    pub asset_id: AssetId,
    pub account: Account,
    pub loading: bool,
}

impl Default for AssetBalanceInputData {
    fn default() -> Self {
        Self {
            class_id: ClassId::from(0),
            asset_id: AssetId::from(0),
            account: Account::from("".to_string()),
            loading: false,
        }
    }
}

pub fn asset_balance_ui(ui: &mut egui::Ui, asset: &mut ResMut<AssetUi>) {
    ui.label("Asset Balance");
    ui.separator();
    ui.label("Class ID");
    ui.add(egui::DragValue::new::<u64>(&mut *asset.data.input.balance.class_id).speed(0.1));
    ui.label("Asset ID");
    ui.add(egui::DragValue::new::<u64>(&mut *asset.data.input.balance.asset_id).speed(0.1));
    ui.label("Account");
    ui.text_edit_singleline(&mut *asset.data.input.balance.account);
    ui.separator();
    if asset.data.input.balance.loading {
        ui.separator();
        ui.add(egui::Spinner::default());
    } else {
        if ui.button("Get Balance").clicked() {
            asset
                .channels
                .balance
                .input_tx
                .send(AssetBalanceRequest {
                    input: AssetBalanceInput {
                        class_id: asset.data.input.balance.class_id.clone(),
                        asset_id: asset.data.input.balance.asset_id.clone(),
                        account: asset.data.input.balance.account.clone(),
                    },
                })
                .unwrap();
        }
    }
    if let Some(output) = &asset.data.output.balance {
        ui.separator();
        ui.label("Amount");
        ui.text_edit_singleline(&mut u128::from(output.amount).to_string());
    }
}

pub fn handle_balance_response(mut asset: ResMut<AssetUi>, tokio_runtime: Res<TokioRuntime>) {
    if let Ok(balance_result) = asset.channels.balance.output_rx.0.try_recv() {
        if let Some(balance) = balance_result {
            asset.data.output.balance = Some(balance);
        }
        asset.data.input.balance.loading = false;
    }

    request_handler::<AssetBalanceRequest, AssetBalanceInput, AssetBalanceOutput>(
        tokio_runtime.runtime.clone(),
        asset.channels.balance.input_rx.clone(),
        asset.channels.balance.output_tx.clone(),
    );
}

pub struct AssetBalancePlugin;

impl Plugin for AssetBalancePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(handle_balance_response);
    }
}
