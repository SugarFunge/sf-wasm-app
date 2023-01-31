use bevy::prelude::*;
use bevy_egui::egui;
use crossbeam::channel;
use sugarfunge_api_types::{
    bundle::{MintBundleInput, MintBundleOutput},
    primitives::{Account, Balance, BundleId, Seed},
};

use crate::{prelude::*, util::request_handler};

use super::BundleUi;

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

#[derive(Resource)]
pub struct MintBundleChannel {
    pub input_tx: InputSender<MintBundleRequest>,
    pub input_rx: InputReceiver<MintBundleRequest>,
    pub output_tx: OutputSender<MintBundleOutput>,
    pub output_rx: OutputReceiver<MintBundleOutput>,
}

impl Default for MintBundleChannel {
    fn default() -> Self {
        let (input_tx, input_rx) = channel::unbounded::<MintBundleRequest>();
        let (output_tx, output_rx) = channel::unbounded::<Option<MintBundleOutput>>();
        Self {
            input_tx: InputSender(input_tx),
            input_rx: InputReceiver(input_rx),
            output_tx: OutputSender(output_tx),
            output_rx: OutputReceiver(output_rx),
        }
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

pub fn mint_bundle_ui(ui: &mut egui::Ui, bundle: &mut ResMut<BundleUi>) {
    ui.label("Mint Bundle");
    ui.separator();
    ui.label("Seed");
    ui.text_edit_singleline(&mut *bundle.data.input.mint.seed);
    ui.label("From");
    ui.text_edit_singleline(&mut *bundle.data.input.mint.from);
    ui.label("To");
    ui.text_edit_singleline(&mut *bundle.data.input.mint.to);
    ui.label("Bundle ID");
    ui.text_edit_singleline(&mut *bundle.data.input.mint.bundle_id);
    ui.label("Amount");
    ui.add(egui::DragValue::new(&mut bundle.data.input.mint.amount).speed(1.0));
    ui.separator();
    if ui.button("Mint").clicked() {
        bundle
            .channels
            .mint
            .input_tx
            .0
            .send(MintBundleRequest {
                input: MintBundleInput {
                    seed: bundle.data.input.mint.seed.clone(),
                    from: bundle.data.input.mint.from.clone(),
                    to: bundle.data.input.mint.to.clone(),
                    bundle_id: bundle.data.input.mint.bundle_id.clone(),
                    amount: Balance::from(bundle.data.input.mint.amount as u128),
                },
            })
            .unwrap();
        bundle.data.input.mint.loading = true;
    }
    ui.separator();
    if let Some(output) = &bundle.data.output.mint {
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
    if let Ok(mint_result) = bundle.channels.mint.output_rx.0.try_recv() {
        if let Some(mint) = mint_result {
            bundle.data.output.mint = Some(mint);
        }
        bundle.data.input.mint.loading = false;
    }

    request_handler::<MintBundleRequest, MintBundleInput, MintBundleOutput>(
        tokio_runtime.runtime.clone(),
        bundle.channels.mint.input_rx.clone(),
        bundle.channels.mint.output_tx.clone(),
    );
}

pub struct MintBundlePlugin;

impl Plugin for MintBundlePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(handle_mint_response);
    }
}
