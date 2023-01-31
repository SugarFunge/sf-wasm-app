use bevy::prelude::*;
use bevy_egui::egui;
use crossbeam::channel;
use sugarfunge_api_types::{
    asset::{TransferFromInput, TransferFromOutput},
    primitives::{Account, AssetId, Balance, ClassId, Seed},
};

use crate::{prelude::*, util::request_handler};

use super::AssetUi;

#[derive(Debug)]
pub struct AssetTransferFromRequest {
    pub input: TransferFromInput,
}

impl Request<TransferFromInput> for AssetTransferFromRequest {
    fn endpoint(&self) -> &str {
        "asset/transfer_from"
    }

    fn input(&self) -> Option<TransferFromInput> {
        Some(TransferFromInput {
            seed: self.input.seed.clone(),
            class_id: self.input.class_id.clone(),
            asset_id: self.input.asset_id.clone(),
            amount: self.input.amount.clone(),
            from: self.input.from.clone(),
            to: self.input.to.clone(),
        })
    }
}

#[derive(Resource)]
pub struct AssetTransferFromChannel {
    pub input_tx: InputSender<AssetTransferFromRequest>,
    pub input_rx: InputReceiver<AssetTransferFromRequest>,
    pub output_tx: OutputSender<TransferFromOutput>,
    pub output_rx: OutputReceiver<TransferFromOutput>,
}

impl Default for AssetTransferFromChannel {
    fn default() -> Self {
        let (input_tx, input_rx) = channel::unbounded::<AssetTransferFromRequest>();
        let (output_tx, output_rx) = channel::unbounded::<Option<TransferFromOutput>>();
        Self {
            input_tx: InputSender(input_tx),
            input_rx: InputReceiver(input_rx),
            output_tx: OutputSender(output_tx),
            output_rx: OutputReceiver(output_rx),
        }
    }
}

#[derive(Resource, Debug, Clone)]
pub struct AssetTransferFromInputData {
    pub seed: Seed,
    pub class_id: ClassId,
    pub asset_id: AssetId,
    pub amount: u64,
    pub from: Account,
    pub to: Account,
    pub loading: bool,
}

impl Default for AssetTransferFromInputData {
    fn default() -> Self {
        Self {
            seed: Seed::from("".to_string()),
            class_id: ClassId::from(0),
            asset_id: AssetId::from(0),
            amount: 0,
            from: Account::from("".to_string()),
            to: Account::from("".to_string()),
            loading: false,
        }
    }
}

pub fn asset_transfer_from_ui(ui: &mut egui::Ui, asset: &mut ResMut<AssetUi>) {
    ui.label("Transfer Asset From");
    ui.separator();
    ui.label("Seed");
    ui.text_edit_singleline(&mut *asset.data.input.transfer_from.seed);
    ui.label("Class ID");
    ui.add(egui::DragValue::new::<u64>(&mut *asset.data.input.transfer_from.class_id).speed(0.1));
    ui.label("Asset ID");
    ui.add(egui::DragValue::new::<u64>(&mut *asset.data.input.transfer_from.asset_id).speed(0.1));
    ui.label("Amount");
    ui.add(egui::DragValue::new::<u64>(&mut asset.data.input.transfer_from.amount).speed(0.1));
    ui.label("From");
    ui.text_edit_singleline(&mut *asset.data.input.transfer_from.from);
    ui.label("To");
    ui.text_edit_singleline(&mut *asset.data.input.transfer_from.to);
    ui.separator();
    if asset.data.input.transfer_from.loading {
        ui.separator();
        ui.add(egui::Spinner::default());
    } else {
        if ui.button("Transfer").clicked() {
            asset.channels.transfer_from.input_tx.0
                .send(AssetTransferFromRequest {
                    input: TransferFromInput {
                        seed: asset.data.input.transfer_from.seed.clone(),
                        class_id: asset.data.input.transfer_from.class_id.clone(),
                        asset_id: asset.data.input.transfer_from.asset_id.clone(),
                        amount: Balance::from(
                            asset.data.input.transfer_from.amount.clone() as u128
                        ),
                        from: asset.data.input.transfer_from.from.clone(),
                        to: asset.data.input.transfer_from.to.clone(),
                    },
                })
                .unwrap();
        }
    }
    if let Some(output) = &asset.data.output.transfer_from {
        ui.separator();
        ui.label("From");
        ui.text_edit_singleline(&mut output.from.as_str());
        ui.label("To");
        ui.text_edit_singleline(&mut output.to.as_str());
        ui.label("Class ID");
        ui.text_edit_singleline(&mut u64::from(output.class_id).to_string());
        ui.label("Asset ID");
        ui.text_edit_singleline(&mut u64::from(output.asset_id).to_string());
        ui.label("Amount");
        ui.text_edit_singleline(&mut u128::from(output.amount).to_string());
        ui.label("Who");
        ui.text_edit_singleline(&mut output.who.as_str());
    }
}

pub fn handle_transfer_from_response(mut asset: ResMut<AssetUi>, tokio_runtime: Res<TokioRuntime>) {
    if let Ok(transfer_from_result) = asset.channels.transfer_from.output_rx.0.try_recv() {
        if let Some(transfer_from) = transfer_from_result {
            asset.data.output.transfer_from = Some(transfer_from);
        }
        asset.data.input.transfer_from.loading = false;
    }

    request_handler::<AssetTransferFromRequest, TransferFromInput, TransferFromOutput>(
        tokio_runtime.runtime.clone(),
        asset.channels.transfer_from.input_rx.clone(),
        asset.channels.transfer_from.output_tx.clone(),
    );
}

pub struct AssetTransferFromPlugin;

impl Plugin for AssetTransferFromPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(handle_transfer_from_response);
    }
}
