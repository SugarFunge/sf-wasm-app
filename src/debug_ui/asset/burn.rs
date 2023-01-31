use bevy::prelude::*;
use bevy_egui::egui;
use crossbeam::channel;
use sugarfunge_api_types::{
    asset::{BurnInput, BurnOutput},
    primitives::{Account, AssetId, Balance, ClassId, Seed},
};

use crate::{prelude::*, util::request_handler};

use super::AssetUi;

#[derive(Debug)]
pub struct AssetBurnRequest {
    pub input: BurnInput,
}

impl Request<BurnInput> for AssetBurnRequest {
    fn endpoint(&self) -> &str {
        "asset/burn"
    }

    fn input(&self) -> Option<BurnInput> {
        Some(BurnInput {
            seed: self.input.seed.clone(),
            class_id: self.input.class_id.clone(),
            asset_id: self.input.asset_id.clone(),
            amount: self.input.amount.clone(),
            from: self.input.from.clone(),
        })
    }
}

#[derive(Resource)]
pub struct AssetBurnChannel {
    pub input_tx: InputSender<AssetBurnRequest>,
    pub input_rx: InputReceiver<AssetBurnRequest>,
    pub output_tx: OutputSender<BurnOutput>,
    pub output_rx: OutputReceiver<BurnOutput>,
}

impl Default for AssetBurnChannel {
    fn default() -> Self {
        let (input_tx, input_rx) = channel::unbounded::<AssetBurnRequest>();
        let (output_tx, output_rx) = channel::unbounded::<Option<BurnOutput>>();
        Self {
            input_tx: InputSender(input_tx),
            input_rx: InputReceiver(input_rx),
            output_tx: OutputSender(output_tx),
            output_rx: OutputReceiver(output_rx),
        }
    }
}

#[derive(Resource, Debug, Clone)]
pub struct AssetBurnInputData {
    pub seed: Seed,
    pub class_id: ClassId,
    pub asset_id: AssetId,
    pub amount: u64,
    pub from: Account,
    pub loading: bool,
}

impl Default for AssetBurnInputData {
    fn default() -> Self {
        Self {
            seed: Seed::from("".to_string()),
            class_id: ClassId::from(0),
            asset_id: AssetId::from(0),
            amount: 0,
            from: Account::from("".to_string()),
            loading: false,
        }
    }
}

pub fn asset_burn_ui(ui: &mut egui::Ui, asset: &mut ResMut<AssetUi>) {
    ui.label("Burn Asset");
    ui.separator();
    ui.label("Seed");
    ui.text_edit_singleline(&mut *asset.data.input.burn.seed);
    ui.label("Class ID");
    ui.add(egui::DragValue::new::<u64>(&mut *asset.data.input.burn.class_id).speed(0.1));
    ui.label("Asset ID");
    ui.add(egui::DragValue::new::<u64>(&mut *asset.data.input.burn.asset_id).speed(0.1));
    ui.label("Amount");
    ui.add(egui::DragValue::new::<u64>(&mut asset.data.input.burn.amount).speed(0.1));
    ui.label("From");
    ui.text_edit_singleline(&mut *asset.data.input.burn.from);
    ui.separator();
    if asset.data.input.burn.loading {
        ui.separator();
        ui.add(egui::Spinner::default());
    } else {
        if ui.button("Burn").clicked() {
            asset
                .channels
                .burn
                .input_tx
                .0
                .send(AssetBurnRequest {
                    input: BurnInput {
                        seed: asset.data.input.burn.seed.clone(),
                        class_id: asset.data.input.burn.class_id,
                        asset_id: asset.data.input.burn.asset_id,
                        amount: Balance::from(asset.data.input.burn.amount as u128),
                        from: asset.data.input.burn.from.clone(),
                    },
                })
                .unwrap();
            asset.data.input.burn.loading = true;
        }
    }
    if let Some(output) = &asset.data.output.burn {
        ui.separator();
        ui.label("From");
        ui.text_edit_singleline(&mut output.from.as_str());
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

pub fn handle_burn_response(mut asset: ResMut<AssetUi>, tokio_runtime: Res<TokioRuntime>) {
    if let Ok(burn_result) = asset.channels.burn.output_rx.0.try_recv() {
        if let Some(burn) = burn_result {
            asset.data.output.burn = Some(burn);
        }
        asset.data.input.burn.loading = false;
    }

    request_handler::<AssetBurnRequest, BurnInput, BurnOutput>(
        tokio_runtime.runtime.clone(),
        asset.channels.burn.input_rx.clone(),
        asset.channels.burn.output_tx.clone(),
    );
}

pub struct AssetBurnPlugin;

impl Plugin for AssetBurnPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(handle_burn_response);
    }
}
