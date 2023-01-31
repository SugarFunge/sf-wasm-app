use bevy::prelude::*;
use bevy_egui::egui;
use crossbeam::channel;
use sugarfunge_api_types::{
    bundle::{BurnBundleInput, BurnBundleOutput},
    primitives::{Account, Balance, BundleId, Seed},
};

use crate::{prelude::*, util::request_handler};

use super::BundleUi;

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

#[derive(Resource)]
pub struct BurnBundleChannel {
    pub input_tx: InputSender<BurnBundleRequest>,
    pub input_rx: InputReceiver<BurnBundleRequest>,
    pub output_tx: OutputSender<BurnBundleOutput>,
    pub output_rx: OutputReceiver<BurnBundleOutput>,
}

impl Default for BurnBundleChannel {
    fn default() -> Self {
        let (input_tx, input_rx) = channel::unbounded::<BurnBundleRequest>();
        let (output_tx, output_rx) = channel::unbounded::<Option<BurnBundleOutput>>();
        Self {
            input_tx: InputSender(input_tx),
            input_rx: InputReceiver(input_rx),
            output_tx: OutputSender(output_tx),
            output_rx: OutputReceiver(output_rx),
        }
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

pub fn burn_bundle_ui(ui: &mut egui::Ui, bundle: &mut ResMut<BundleUi>) {
    ui.label("Burn Bundle");
    ui.separator();
    ui.label("Seed");
    ui.text_edit_singleline(&mut *bundle.data.input.burn.seed);
    ui.label("From");
    ui.text_edit_singleline(&mut *bundle.data.input.burn.from);
    ui.label("To");
    ui.text_edit_singleline(&mut *bundle.data.input.burn.to);
    ui.label("Bundle ID");
    ui.text_edit_singleline(&mut *bundle.data.input.burn.bundle_id);
    ui.label("Amount");
    ui.add(egui::DragValue::new(&mut bundle.data.input.burn.amount).speed(1.0));
    ui.separator();
    if ui.button("Burn").clicked() {
        bundle
            .channels
            .burn
            .input_tx
            .0
            .send(BurnBundleRequest {
                input: BurnBundleInput {
                    seed: bundle.data.input.burn.seed.clone(),
                    from: bundle.data.input.burn.from.clone(),
                    to: bundle.data.input.burn.to.clone(),
                    bundle_id: bundle.data.input.burn.bundle_id.clone(),
                    amount: Balance::from(bundle.data.input.burn.amount as u128),
                },
            })
            .unwrap();
        bundle.data.input.burn.loading = true;
    }
    ui.separator();
    if let Some(output) = &bundle.data.output.burn {
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

pub fn handle_mint_response(mut bundle: ResMut<BundleUi>, tokio_runtime: Res<TokioRuntime>) {
    if let Ok(burn_result) = bundle.channels.burn.output_rx.0.try_recv() {
        if let Some(burn) = burn_result {
            bundle.data.output.burn = Some(burn);
        }
        bundle.data.input.burn.loading = false;
    }

    request_handler::<BurnBundleRequest, BurnBundleInput, BurnBundleOutput>(
        tokio_runtime.runtime.clone(),
        bundle.channels.burn.input_rx.clone(),
        bundle.channels.burn.output_tx.clone(),
    );
}

pub struct BurnBundlePlugin;

impl Plugin for BurnBundlePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(handle_mint_response);
    }
}
