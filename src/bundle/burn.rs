use bevy::prelude::*;
use bevy_egui::egui;
use sugarfunge_api_types::{
    bundle::{BurnBundleInput, BurnBundleOutput},
    primitives::{Account, Balance, BundleId, Seed},
};

use crate::{prelude::*, util::*};

use super::{BundleInputData, BundleOutputData};

#[derive(Debug)]
pub struct BurnBundleRequest {
    pub input: BurnBundleInput,
}

impl Request<BurnBundleInput> for BurnBundleRequest {
    fn endpoint(&self) -> &str {
        "bundle/burn"
    }

    fn input(&self) -> Option<BurnBundleInput> {
        Some(BurnBundleInput {
            seed: self.input.seed.clone(),
            from: self.input.from.clone(),
            to: self.input.to.clone(),
            bundle_id: self.input.bundle_id.clone(),
            amount: self.input.amount.clone(),
        })
    }
}

#[derive(Resource, Debug, Clone)]
pub struct BurnBundleInputData {
    pub seed: Seed,
    pub from: Account,
    pub to: Account,
    pub bundle_id: BundleId,
    pub amount: u64,
    pub loading: bool,
}

impl Default for BurnBundleInputData {
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

pub fn burn_bundle_ui(
    ui: &mut egui::Ui,
    bundle_input: &mut ResMut<BundleInputData>,
    burn_tx: &Res<InputSender<BurnBundleRequest>>,
    bundle_output: &Res<BundleOutputData>,
) {
    ui.label("Burn Bundle");
    ui.separator();
    ui.label("Seed");
    ui.text_edit_singleline(&mut *bundle_input.burn_input.seed);
    ui.label("From");
    ui.text_edit_singleline(&mut *bundle_input.burn_input.from);
    ui.label("To");
    ui.text_edit_singleline(&mut *bundle_input.burn_input.to);
    ui.label("Bundle ID");
    ui.text_edit_singleline(&mut *bundle_input.burn_input.bundle_id);
    ui.label("Amount");
    ui.label("The Amounts are represented in 10^18 units.");
    ui.add(egui::DragValue::new(&mut bundle_input.burn_input.amount).speed(1.0));
    ui.separator();
    if ui.button("Burn").clicked() {
        burn_tx
            .send(BurnBundleRequest {
                input: BurnBundleInput {
                    seed: bundle_input.burn_input.seed.clone(),
                    from: bundle_input.burn_input.from.clone(),
                    to: bundle_input.burn_input.to.clone(),
                    bundle_id: bundle_input.burn_input.bundle_id.clone(),
                    amount: Balance::from(
                        (bundle_input.burn_input.amount as u128) * u128::pow(10, 18),
                    ),
                },
            })
            .unwrap();
        bundle_input.burn_input.loading = true;
    }
    ui.separator();
    if let Some(output) = &bundle_output.burn_output {
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
    burn_rx: Res<OutputReceiver<BurnBundleOutput>>,
) {
    if let Ok(burn_result) = burn_rx.0.try_recv() {
        if let Some(burn) = burn_result {
            bundle_output.burn_output = Some(burn);
        }
        bundle_input.burn_input.loading = false;
    }
}

pub struct BurnBundlePlugin;

impl Plugin for BurnBundlePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_in_out_channels::<BurnBundleRequest, BurnBundleOutput>)
            .add_system(request_handler::<BurnBundleRequest, BurnBundleInput, BurnBundleOutput>)
            .add_system(handle_mint_response);
    }
}
