use bevy::prelude::*;
use bevy_egui::egui;
use sugarfunge_api_types::{
    asset::{MintInput, MintOutput},
    primitives::{Account, AssetId, Balance, ClassId, Seed},
};

use crate::{
    prelude::*,
    util::{request_handler, setup_in_out_channels},
};

use super::{AssetInputData, AssetOutputData};

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

pub fn asset_mint_ui(
    ui: &mut egui::Ui,
    asset_input: &mut ResMut<AssetInputData>,
    minted_tx: &Res<InputSender<AssetMintRequest>>,
    asset_output: &Res<AssetOutputData>,
) {
    ui.label("Mint Asset");
    ui.separator();
    ui.label("Seed");
    ui.text_edit_singleline(&mut *asset_input.mint_input.seed);
    ui.label("Class ID");
    ui.add(egui::DragValue::new::<u64>(&mut *asset_input.mint_input.class_id).speed(0.1));
    ui.label("Asset ID");
    ui.add(egui::DragValue::new::<u64>(&mut *asset_input.mint_input.asset_id).speed(0.1));
    ui.label("Amount");
    ui.add(egui::DragValue::new::<u64>(&mut asset_input.mint_input.amount).speed(0.1));
    ui.label("To");
    ui.text_edit_singleline(&mut *asset_input.mint_input.to);
    if asset_input.mint_input.loading {
        ui.separator();
        ui.add(egui::Spinner::default());
    } else {
        if ui.button("Mint").clicked() {
            minted_tx
                .send(AssetMintRequest {
                    input: MintInput {
                        seed: asset_input.mint_input.seed.clone(),
                        class_id: asset_input.mint_input.class_id,
                        to: asset_input.mint_input.to.clone(),
                        asset_id: asset_input.mint_input.asset_id,
                        amount: Balance::from(asset_input.mint_input.amount as u128),
                    },
                })
                .unwrap();
            asset_input.mint_input.loading = true;
        }
    }
    if let Some(output) = &asset_output.mint_output {
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

pub fn handle_mint_response(
    mut asset_output: ResMut<AssetOutputData>,
    mut asset_input: ResMut<AssetInputData>,
    mint_rx: Res<OutputReceiver<MintOutput>>,
) {
    if let Ok(mint_result) = mint_rx.0.try_recv() {
        if let Some(mint) = mint_result {
            asset_output.mint_output = Some(mint);
        }
        asset_input.mint_input.loading = false;
    }
}

pub struct AssetMintPlugin;

impl Plugin for AssetMintPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_in_out_channels::<AssetMintRequest, MintOutput>)
            .add_system(request_handler::<AssetMintRequest, MintInput, MintOutput>)
            .add_system(handle_mint_response);
    }
}
