use bevy::prelude::*;
use bevy_egui::egui;
use sugarfunge_api_types::{
    asset::{TransferFromInput, TransferFromOutput},
    primitives::{Account, AssetId, Balance, ClassId, Seed},
};

use crate::{
    prelude::*,
    util::{request_handler, setup_in_out_channels},
};

use super::{AssetInputData, AssetOutputData};

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

pub fn asset_transfer_from_ui(
    ui: &mut egui::Ui,
    asset_input: &mut ResMut<AssetInputData>,
    transfered_tx: &Res<InputSender<AssetTransferFromRequest>>,
    asset_output: &Res<AssetOutputData>,
) {
    ui.label("Transfer Asset From");
    ui.separator();
    ui.label("Seed");
    ui.text_edit_singleline(&mut *asset_input.transfer_from_input.seed);
    ui.label("Class ID");
    ui.add(egui::DragValue::new::<u64>(&mut *asset_input.transfer_from_input.class_id).speed(0.1));
    ui.label("Asset ID");
    ui.add(egui::DragValue::new::<u64>(&mut *asset_input.transfer_from_input.asset_id).speed(0.1));
    ui.label("Amount");
    ui.add(egui::DragValue::new::<u64>(&mut asset_input.transfer_from_input.amount).speed(0.1));
    ui.label("From");
    ui.text_edit_singleline(&mut *asset_input.transfer_from_input.from);
    ui.label("To");
    ui.text_edit_singleline(&mut *asset_input.transfer_from_input.to);
    ui.separator();
    if asset_input.transfer_from_input.loading {
        ui.separator();
        ui.add(egui::Spinner::default());
    } else {
        if ui.button("Transfer").clicked() {
            transfered_tx
                .send(AssetTransferFromRequest {
                    input: TransferFromInput {
                        seed: asset_input.transfer_from_input.seed.clone(),
                        class_id: asset_input.transfer_from_input.class_id.clone(),
                        asset_id: asset_input.transfer_from_input.asset_id.clone(),
                        amount: Balance::from(
                            asset_input.transfer_from_input.amount.clone() as u128
                        ),
                        from: asset_input.transfer_from_input.from.clone(),
                        to: asset_input.transfer_from_input.to.clone(),
                    },
                })
                .unwrap();
        }
    }
    if let Some(output) = &asset_output.transfer_from_output {
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

pub fn handle_transfer_from_response(
    mut asset_output: ResMut<AssetOutputData>,
    mut asset_input: ResMut<AssetInputData>,
    transfer_from_rx: Res<OutputReceiver<TransferFromOutput>>,
) {
    if let Ok(transfer_from_result) = transfer_from_rx.0.try_recv() {
        if let Some(transfer_from) = transfer_from_result {
            asset_output.transfer_from_output = Some(transfer_from);
        }
        asset_input.transfer_from_input.loading = false;
    }
}

pub struct AssetTransferFromPlugin;

impl Plugin for AssetTransferFromPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(
            setup_in_out_channels::<AssetTransferFromRequest, TransferFromOutput>,
        )
        .add_system(
            request_handler::<AssetTransferFromRequest, TransferFromInput, TransferFromOutput>,
        )
        .add_system(handle_transfer_from_response);
    }
}
