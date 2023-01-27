use bevy::prelude::*;
use bevy_egui::egui;
use sugarfunge_api_types::{
    bag::{SweepInput, SweepOutput},
    primitives::{Account, Seed},
};

use crate::{
    prelude::*,
    util::{request_handler, setup_in_out_channels},
};

use super::{BagInputData, BagOutputData};

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

pub fn sweep_bag_ui(
    ui: &mut egui::Ui,
    bag_input: &mut ResMut<BagInputData>,
    sweep_tx: &Res<InputSender<SweepBagRequest>>,
    bag_output: &Res<BagOutputData>,
) {
    ui.label("Sweep Bag");
    ui.separator();
    ui.label("Seed");
    ui.text_edit_singleline(&mut *bag_input.sweep_input.seed);
    ui.label("Bag");
    ui.text_edit_singleline(&mut *bag_input.sweep_input.bag);
    ui.label("To");
    ui.text_edit_singleline(&mut *bag_input.sweep_input.to);
    if bag_input.sweep_input.loading {
        ui.separator();
        ui.add(egui::Spinner::default());
    } else {
        if ui.button("Sweep").clicked() {
            sweep_tx
                .send(SweepBagRequest {
                    input: SweepInput {
                        seed: bag_input.sweep_input.seed.clone(),
                        bag: bag_input.sweep_input.bag.clone(),
                        to: bag_input.sweep_input.to.clone(),
                    },
                })
                .unwrap();
            bag_input.sweep_input.loading = true;
        }
    }
    if let Some(output) = &bag_output.sweep_output {
        ui.separator();
        ui.label("Bag");
        ui.text_edit_singleline(&mut output.bag.as_str());
        ui.label("Who");
        ui.text_edit_singleline(&mut output.who.as_str());
        ui.label("To");
        ui.text_edit_singleline(&mut output.to.as_str());
    }
}

pub fn handle_sweep_response(
    mut bag_output: ResMut<BagOutputData>,
    mut bag_input: ResMut<BagInputData>,
    swept_rx: Res<OutputReceiver<SweepOutput>>,
) {
    if let Ok(swept_result) = swept_rx.0.try_recv() {
        if let Some(swept) = swept_result {
            bag_output.sweep_output = Some(swept);
        }
        bag_input.sweep_input.loading = false;
    }
}

pub struct SweepBagPlugin;

impl Plugin for SweepBagPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_in_out_channels::<SweepBagRequest, SweepOutput>)
            .add_system(request_handler::<SweepBagRequest, SweepInput, SweepOutput>)
            .add_system(handle_sweep_response);
    }
}
