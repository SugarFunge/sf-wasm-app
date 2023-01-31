use bevy::prelude::*;
use bevy_egui::egui;
use crossbeam::channel;
use sugarfunge_api_types::{
    bag::{SweepInput, SweepOutput},
    primitives::{Account, Seed},
};

use crate::{prelude::*, util::request_handler};

use super::BagUi;

#[derive(Debug)]
pub struct SweepBagRequest {
    pub input: SweepInput,
}

impl Request<SweepInput> for SweepBagRequest {
    fn endpoint(&self) -> &str {
        "bag/sweep"
    }

    fn input(&self) -> Option<SweepInput> {
        Some(SweepInput {
            seed: self.input.seed.clone(),
            bag: self.input.bag.clone(),
            to: self.input.to.clone(),
        })
    }
}

#[derive(Resource)]
pub struct SweepBagChannel {
    pub input_tx: InputSender<SweepBagRequest>,
    pub input_rx: InputReceiver<SweepBagRequest>,
    pub output_tx: OutputSender<SweepOutput>,
    pub output_rx: OutputReceiver<SweepOutput>,
}

impl Default for SweepBagChannel {
    fn default() -> Self {
        let (input_tx, input_rx) = channel::unbounded::<SweepBagRequest>();
        let (output_tx, output_rx) = channel::unbounded::<Option<SweepOutput>>();
        Self {
            input_tx: InputSender(input_tx),
            input_rx: InputReceiver(input_rx),
            output_tx: OutputSender(output_tx),
            output_rx: OutputReceiver(output_rx),
        }
    }
}

#[derive(Resource, Debug, Clone)]
pub struct SweepBagInputData {
    pub seed: Seed,
    pub bag: Account,
    pub to: Account,
    pub loading: bool,
}

impl Default for SweepBagInputData {
    fn default() -> Self {
        Self {
            seed: Seed::from("".to_string()),
            bag: Account::from("".to_string()),
            to: Account::from("".to_string()),
            loading: false,
        }
    }
}

pub fn sweep_bag_ui(ui: &mut egui::Ui, bag: &mut ResMut<BagUi>) {
    ui.label("Sweep Bag");
    ui.separator();
    ui.label("Seed");
    ui.text_edit_singleline(&mut *bag.data.input.sweep.seed);
    ui.label("Bag");
    ui.text_edit_singleline(&mut *bag.data.input.sweep.bag);
    ui.label("To");
    ui.text_edit_singleline(&mut *bag.data.input.sweep.to);
    if bag.data.input.sweep.loading {
        ui.separator();
        ui.add(egui::Spinner::default());
    } else {
        if ui.button("Sweep").clicked() {
            bag.channels
                .sweep
                .input_tx
                .0
                .send(SweepBagRequest {
                    input: SweepInput {
                        seed: bag.data.input.sweep.seed.clone(),
                        bag: bag.data.input.sweep.bag.clone(),
                        to: bag.data.input.sweep.to.clone(),
                    },
                })
                .unwrap();
            bag.data.input.sweep.loading = true;
        }
    }
    if let Some(output) = &bag.data.output.sweep {
        ui.separator();
        ui.label("Bag");
        ui.text_edit_singleline(&mut output.bag.as_str());
        ui.label("Who");
        ui.text_edit_singleline(&mut output.who.as_str());
        ui.label("To");
        ui.text_edit_singleline(&mut output.to.as_str());
    }
}

pub fn handle_sweep_response(mut bag: ResMut<BagUi>, tokio_runtime: Res<TokioRuntime>) {
    if let Ok(swept_result) = bag.channels.sweep.output_rx.0.try_recv() {
        if let Some(swept) = swept_result {
            bag.data.output.sweep = Some(swept);
        }
        bag.data.input.sweep.loading = false;
    }

    request_handler::<SweepBagRequest, SweepInput, SweepOutput>(
        tokio_runtime.runtime.clone(),
        bag.channels.sweep.input_rx.clone(),
        bag.channels.sweep.output_tx.clone(),
    );
}

pub struct SweepBagPlugin;

impl Plugin for SweepBagPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(handle_sweep_response);
    }
}
