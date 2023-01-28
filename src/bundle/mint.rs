use bevy::prelude::*;
use bevy_egui::egui;
use sugarfunge_api_types::{
    bundle::{MintBundleInput, MintBundleOutput},
    primitives::{Account, Balance, BundleId, Seed},
};

use crate::{prelude::*, util::*};

use super::{BundleInputData, BundleOutputData};

#[derive(Debug)]
pub struct MintBundleRequest {
    pub input: MintBundleInput,
}

impl Request<MintBundleInput> for MintBundleRequest {
    fn endpoint(&self) -> &str {
        "bundle/mint"
    }

    fn input(&self) -> Option<MintBundleInput> {
        Some(MintBundleInput {
            seed: self.input.seed.clone(),
            from: self.input.from.clone(),
            to: self.input.to.clone(),
            bundle_id: self.input.bundle_id.clone(),
            amount: self.input.amount.clone(),
        })
    }
}

#[derive(Resource, Debug, Clone)]
pub struct MintBundleInputData {
    pub seed: Seed,
    pub from: Account,
    pub to: Account,
    pub bundle_id: BundleId,
    pub amount: u64,
    pub loading: bool,
}

impl Default for MintBundleInputData {
    fn default() -> Self {
        Self {
            seed: Seed::from("".to_string()),
            from: Account::from("".to_string()),
            to: Account::from("".to_string()),
            bundle_id: BundleId::from("".to_string()),
            amount: 0,
            loading: false,
        }
    }
}

pub fn mint_bundle_ui(
    ui: &mut egui::Ui,
    bundle_input: &mut ResMut<BundleInputData>,
    mint_tx: &Res<InputSender<MintBundleRequest>>,
    bundle_output: &Res<BundleOutputData>,
) {
    ui.label("Mint Bundle");
    ui.separator();
    ui.label("Seed");
    ui.text_edit_singleline(&mut *bundle_input.mint_input.seed);
    ui.label("From");
    ui.text_edit_singleline(&mut *bundle_input.mint_input.from);
    ui.label("To");
    ui.text_edit_singleline(&mut *bundle_input.mint_input.to);
    ui.label("Bundle ID");
    ui.text_edit_singleline(&mut *bundle_input.mint_input.bundle_id);
    ui.label("Amount");
    ui.add(egui::DragValue::new(&mut bundle_input.mint_input.amount).speed(1.0));
    ui.separator();
    if ui.button("Mint").clicked() {
        mint_tx
            .send(MintBundleRequest {
                input: MintBundleInput {
                    seed: bundle_input.mint_input.seed.clone(),
                    from: bundle_input.mint_input.from.clone(),
                    to: bundle_input.mint_input.to.clone(),
                    bundle_id: bundle_input.mint_input.bundle_id.clone(),
                    amount: Balance::from(bundle_input.mint_input.amount as u128),
                },
            })
            .unwrap();
        bundle_input.mint_input.loading = true;
    }
    ui.separator();
    if let Some(output) = &bundle_output.mint_output {
        ui.separator();
        ui.label("Who");
        ui.text_edit_singleline(&mut output.who.as_str());
        ui.label("From");
        ui.text_edit_singleline(&mut output.from.as_str());
        ui.label("To");
        ui.text_edit_singleline(&mut output.to.as_str());
        ui.label("Bundle ID");
        ui.text_edit_singleline(&mut output.bundle_id.as_str());
        ui.label("Amount");
        ui.text_edit_singleline(&mut u128::from(output.amount).to_string());
    }
}

pub fn handle_mint_response(
    mut bundle_output: ResMut<BundleOutputData>,
    mut bundle_input: ResMut<BundleInputData>,
    mint_rx: Res<OutputReceiver<MintBundleOutput>>,
) {
    if let Ok(mint_result) = mint_rx.0.try_recv() {
        if let Some(mint) = mint_result {
            bundle_output.mint_output = Some(mint);
        }
        bundle_input.mint_input.loading = false;
    }
}

pub struct MintBundlePlugin;

impl Plugin for MintBundlePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_in_out_channels::<MintBundleRequest, MintBundleOutput>)
            .add_system(request_handler::<MintBundleRequest, MintBundleInput, MintBundleOutput>)
            .add_system(handle_mint_response);
    }
}
