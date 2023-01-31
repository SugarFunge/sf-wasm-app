use bevy::prelude::*;
use bevy_egui::egui;
use crossbeam::channel;
use sugarfunge_api_types::{
    asset::{AssetBalancesInput, AssetBalancesOutput},
    primitives::{Account, ClassId},
};

use crate::{prelude::*, util::request_handler};

use super::AssetUi;

#[derive(Debug)]
pub struct AssetBalancesRequest {
    pub input: AssetBalancesInput,
}

impl Request<AssetBalancesInput> for AssetBalancesRequest {
    fn endpoint(&self) -> &str {
        "asset/balances"
    }

    fn input(&self) -> Option<AssetBalancesInput> {
        Some(AssetBalancesInput {
            class_id: self.input.class_id.clone(),
            account: self.input.account.clone(),
        })
    }
}

#[derive(Resource)]
pub struct AssetBalancesChannel {
    pub input_tx: InputSender<AssetBalancesRequest>,
    pub input_rx: InputReceiver<AssetBalancesRequest>,
    pub output_tx: OutputSender<AssetBalancesOutput>,
    pub output_rx: OutputReceiver<AssetBalancesOutput>,
}

impl Default for AssetBalancesChannel {
    fn default() -> Self {
        let (input_tx, input_rx) = channel::unbounded::<AssetBalancesRequest>();
        let (output_tx, output_rx) = channel::unbounded::<Option<AssetBalancesOutput>>();
        Self {
            input_tx: InputSender(input_tx),
            input_rx: InputReceiver(input_rx),
            output_tx: OutputSender(output_tx),
            output_rx: OutputReceiver(output_rx),
        }
    }
}

#[derive(Resource, Debug, Clone)]
pub struct AssetBalancesInputData {
    pub class_id: ClassId,
    pub class_id_enabled: bool,
    pub account: Account,
    pub loading: bool,
}

impl Default for AssetBalancesInputData {
    fn default() -> Self {
        Self {
            class_id: ClassId::from(0),
            class_id_enabled: false,
            account: Account::from("".to_string()),
            loading: false,
        }
    }
}

pub fn asset_balances_ui(ui: &mut egui::Ui, asset: &mut ResMut<AssetUi>) {
    ui.label("Asset Balances");
    ui.separator();
    ui.label("Account");
    ui.text_edit_singleline(&mut *asset.data.input.balances.account);
    ui.checkbox(
        &mut asset.data.input.balances.class_id_enabled,
        "Enable Class ID",
    );
    if asset.data.input.balances.class_id_enabled {
        ui.label("Class ID");
        ui.add(egui::DragValue::new::<u64>(&mut *asset.data.input.balances.class_id).speed(0.1));
    }
    ui.separator();
    if asset.data.input.balances.loading {
        ui.separator();
        ui.add(egui::Spinner::default());
    } else {
        if ui.button("Get Balances").clicked() {
            asset
                .channels
                .balances
                .input_tx
                .0
                .send(AssetBalancesRequest {
                    input: AssetBalancesInput {
                        class_id: if asset.data.input.balances.class_id_enabled {
                            Some(asset.data.input.balances.class_id)
                        } else {
                            None
                        },
                        account: asset.data.input.balances.account.clone(),
                    },
                })
                .unwrap();
            asset.data.input.balances.loading = true;
        }
    }
    ui.separator();
    if let Some(balances_output) = &asset.data.output.balances {
        ui.label("Balances");
        ui.separator();
        for (i, balance) in balances_output.balances.iter().enumerate() {
            ui.label(format!("Amount [{}]", i + 1));
            ui.text_edit_singleline(&mut u128::from(balance.amount).to_string());
            ui.separator();
        }
    }
}

pub fn handle_balances_response(mut asset: ResMut<AssetUi>, tokio_runtime: Res<TokioRuntime>) {
    if let Ok(balances_result) = asset.channels.balances.output_rx.0.try_recv() {
        if let Some(balances) = balances_result {
            asset.data.output.balances = Some(balances);
        }
        asset.data.input.balances.loading = false;
    }

    request_handler::<AssetBalancesRequest, AssetBalancesInput, AssetBalancesOutput>(
        tokio_runtime.runtime.clone(),
        asset.channels.balances.input_rx.clone(),
        asset.channels.balances.output_tx.clone(),
    );
}

pub struct AssetBalancesPlugin;

impl Plugin for AssetBalancesPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(handle_balances_response);
    }
}
