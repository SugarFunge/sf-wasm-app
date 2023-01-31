use bevy::prelude::*;
use bevy_egui::egui;
use crossbeam::channel;
use sugarfunge_api_types::{
    asset::{MintInput, MintOutput},
    primitives::{Account, AssetId, Balance, ClassId, Seed},
};

use crate::{prelude::*, util::request_handler};

use super::AssetUi;

#[derive(Debug)]
pub struct AssetMintRequest {
    pub input: MintInput,
}

impl Request<MintInput> for AssetMintRequest {
    fn endpoint(&self) -> &str {
        "asset/mint"
    }

    fn input(&self) -> Option<MintInput> {
        Some(MintInput {
            seed: self.input.seed.clone(),
            class_id: self.input.class_id.clone(),
            to: self.input.to.clone(),
            asset_id: self.input.asset_id.clone(),
            amount: self.input.amount.clone(),
        })
    }
}

#[derive(Resource, Debug, Clone)]
pub struct AssetMintInputData {
    pub seed: Seed,
    pub class_id: ClassId,
    pub asset_id: AssetId,
    pub amount: u64,
    pub to: Account,
    pub loading: bool,
}

impl Default for AssetMintInputData {
    fn default() -> Self {
        Self {
            seed: Seed::from("".to_string()),
            class_id: ClassId::from(0),
            asset_id: AssetId::from(0),
            amount: 0,
            to: Account::from("".to_string()),
            loading: false,
        }
    }
}

#[derive(Resource)]
pub struct AssetMintChannel {
    pub input_tx: InputSender<AssetMintRequest>,
    pub input_rx: InputReceiver<AssetMintRequest>,
    pub output_tx: OutputSender<MintOutput>,
    pub output_rx: OutputReceiver<MintOutput>,
}

impl Default for AssetMintChannel {
    fn default() -> Self {
        let (input_tx, input_rx) = channel::unbounded::<AssetMintRequest>();
        let (output_tx, output_rx) = channel::unbounded::<Option<MintOutput>>();
        Self {
            input_tx: InputSender(input_tx),
            input_rx: InputReceiver(input_rx),
            output_tx: OutputSender(output_tx),
            output_rx: OutputReceiver(output_rx),
        }
    }
}

pub fn asset_mint_ui(ui: &mut egui::Ui, asset: &mut ResMut<AssetUi>) {
    ui.label("Mint Asset");
    ui.separator();
    ui.label("Seed");
    ui.text_edit_singleline(&mut *asset.data.input.mint.seed);
    ui.label("Class ID");
    ui.add(egui::DragValue::new::<u64>(&mut *asset.data.input.mint.class_id).speed(0.1));
    ui.label("Asset ID");
    ui.add(egui::DragValue::new::<u64>(&mut *asset.data.input.mint.asset_id).speed(0.1));
    ui.label("Amount");
    ui.add(egui::DragValue::new::<u64>(&mut asset.data.input.mint.amount).speed(0.1));
    ui.label("To");
    ui.text_edit_singleline(&mut *asset.data.input.mint.to);
    if asset.data.input.mint.loading {
        ui.separator();
        ui.add(egui::Spinner::default());
    } else {
        if ui.button("Mint").clicked() {
            asset
                .channels
                .mint
                .input_tx
                .0
                .send(AssetMintRequest {
                    input: MintInput {
                        seed: asset.data.input.mint.seed.clone(),
                        class_id: asset.data.input.mint.class_id,
                        to: asset.data.input.mint.to.clone(),
                        asset_id: asset.data.input.mint.asset_id,
                        amount: Balance::from(asset.data.input.mint.amount as u128),
                    },
                })
                .unwrap();
            asset.data.input.mint.loading = true;
        }
    }
    if let Some(output) = &asset.data.output.mint {
        ui.separator();
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

pub fn handle_mint_response(mut asset: ResMut<AssetUi>, tokio_runtime: Res<TokioRuntime>) {
    if let Ok(mint_result) = asset.channels.mint.output_rx.try_recv() {
        if let Some(mint) = mint_result {
            asset.data.output.mint = Some(mint);
        }
        asset.data.input.mint.loading = false;
    }

    request_handler::<AssetMintRequest, MintInput, MintOutput>(
        tokio_runtime.runtime.clone(),
        asset.channels.mint.input_rx.clone(),
        asset.channels.mint.output_tx.clone(),
    );
}

pub struct AssetMintPlugin;

impl Plugin for AssetMintPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(handle_mint_response);
    }
}
