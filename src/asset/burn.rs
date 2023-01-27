use bevy::prelude::*;
use bevy_egui::egui;
use sugarfunge_api_types::{
    asset::{BurnInput, BurnOutput},
    primitives::{Account, AssetId, Balance, ClassId, Seed},
};

use crate::{
    prelude::*,
    util::{request_handler, setup_in_out_channels},
};

use super::{AssetInputData, AssetOutputData};

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

pub fn asset_burn_ui(
    ui: &mut egui::Ui,
    asset_input: &mut ResMut<AssetInputData>,
    burned_tx: &Res<InputSender<AssetBurnRequest>>,
    asset_output: &Res<AssetOutputData>,
) {
    ui.label("Burn Asset");
    ui.separator();
    ui.label("Seed");
    ui.text_edit_singleline(&mut *asset_input.burn_input.seed);
    ui.label("Class ID");
    ui.add(egui::DragValue::new::<u64>(&mut *asset_input.burn_input.class_id).speed(0.1));
    ui.label("Asset ID");
    ui.add(egui::DragValue::new::<u64>(&mut *asset_input.burn_input.asset_id).speed(0.1));
    ui.label("Amount");
    ui.add(egui::DragValue::new::<u64>(&mut asset_input.burn_input.amount).speed(0.1));
    ui.label("From");
    ui.text_edit_singleline(&mut *asset_input.burn_input.from);
    ui.separator();
    if asset_input.burn_input.loading {
        ui.separator();
        ui.add(egui::Spinner::default());
    } else {
        if ui.button("Burn").clicked() {
            burned_tx
                .send(AssetBurnRequest {
                    input: BurnInput {
                        seed: asset_input.burn_input.seed.clone(),
                        class_id: asset_input.burn_input.class_id,
                        asset_id: asset_input.burn_input.asset_id,
                        amount: Balance::from(asset_input.burn_input.amount as u128),
                        from: asset_input.burn_input.from.clone(),
                    },
                })
                .unwrap();
            asset_input.burn_input.loading = true;
        }
    }
    if let Some(output) = &asset_output.burn_output {
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

pub fn handle_burn_response(
    mut asset_output: ResMut<AssetOutputData>,
    mut asset_input: ResMut<AssetInputData>,
    burn_rx: Res<OutputReceiver<BurnOutput>>,
) {
    if let Ok(burn_result) = burn_rx.0.try_recv() {
        if let Some(burn) = burn_result {
            asset_output.burn_output = Some(burn);
        }
        asset_input.burn_input.loading = false;
    }
}

pub struct AssetBurnPlugin;

impl Plugin for AssetBurnPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_in_out_channels::<AssetBurnRequest, BurnOutput>)
            .add_system(request_handler::<AssetBurnRequest, BurnInput, BurnOutput>)
            .add_system(handle_burn_response);
    }
}
