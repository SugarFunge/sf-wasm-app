use bevy::prelude::*;
use bevy_egui::egui;
use sugarfunge_api_types::{
    bag::{CreateInput, CreateOutput},
    primitives::{transform_vec_string_to_account, Balance, ClassId, Seed},
};

use crate::{
    prelude::*,
    util::{request_handler, setup_in_out_channels, vec_u64_input_ui},
};

use super::{BagInputData, BagOutputData};

#[derive(Debug)]
pub struct CreateBagRequest {
    pub input: CreateInput,
}

impl Request<CreateInput> for CreateBagRequest {
    fn endpoint(&self) -> &str {
        "bag/create"
    }

    fn input(&self) -> Option<CreateInput> {
        Some(CreateInput {
            seed: self.input.seed.clone(),
            class_id: self.input.class_id.clone(),
            owners: self.input.owners.clone(),
            shares: self.input.shares.clone(),
        })
    }
}

#[derive(Resource, Debug, Default, Clone)]
pub struct CreateBagInputData {
    pub seed: String,
    pub class_id: u64,
    pub owners: Vec<String>,
    pub shares: Vec<u64>,
    pub loading: bool,
}

pub fn create_bag_ui(
    ui: &mut egui::Ui,
    bag_input: &mut ResMut<BagInputData>,
    created_tx: &Res<InputSender<CreateBagRequest>>,
    bag_output: &Res<BagOutputData>,
) {
    ui.label("Create Bag");
    ui.separator();
    ui.label("Seed");
    ui.text_edit_singleline(&mut bag_input.create_input.seed);
    ui.label("Class ID");
    ui.add(egui::DragValue::new::<u64>(&mut bag_input.create_input.class_id).speed(0.1));
    ui.label("Owners");
    if ui.button("Add Owner").clicked() {
        bag_input.create_input.owners.push(String::default());
    }
    let owners = bag_input.create_input.owners.clone();
    let mut owner_remove_index: Option<usize> = None;
    for (i, _) in owners.iter().enumerate() {
        ui.horizontal(|ui| {
            ui.text_edit_singleline(&mut bag_input.create_input.owners[i]);
            if ui.button("Remove").clicked() {
                owner_remove_index = Some(i);
            }
        });
    }
    if let Some(index) = owner_remove_index {
        bag_input.create_input.owners.remove(index);
    }
    ui.label("Shares");
    ui.label("The Shares are represented in 10^18 units.");
    vec_u64_input_ui(ui, &mut bag_input.create_input.shares);
    if bag_input.create_input.loading {
        ui.separator();
        ui.add(egui::Spinner::default());
    } else {
        if ui.button("Create").clicked() {
            let mut shares_input = Vec::new();
            for share in bag_input.create_input.shares.iter() {
                shares_input.push(Balance::from((*share as u128) * (u128::pow(10, 18))));
            }
            created_tx
                .send(CreateBagRequest {
                    input: CreateInput {
                        seed: Seed::from(bag_input.create_input.seed.clone()),
                        class_id: ClassId::from(bag_input.create_input.class_id),
                        owners: transform_vec_string_to_account(
                            bag_input.create_input.owners.clone(),
                        ),
                        shares: shares_input,
                    },
                })
                .unwrap();
            bag_input.create_input.loading = true;
        }
    }
    if let Some(output) = &bag_output.create_output {
        ui.separator();
        ui.label("Bag");
        ui.text_edit_singleline(&mut output.bag.as_str());
        ui.label("Class ID");
        ui.text_edit_singleline(&mut u64::from(output.class_id).to_string());
        ui.label("Asset ID");
        ui.text_edit_singleline(&mut u64::from(output.asset_id).to_string());
        ui.label("Owners");
        for (i, owner) in output.owners.iter().enumerate() {
            ui.label(format!("Account [{}]", i + 1));
            ui.text_edit_singleline(&mut owner.as_str());
            ui.separator();
        }
    }
}

pub fn handle_create_response(
    mut bag_output: ResMut<BagOutputData>,
    mut bag_input: ResMut<BagInputData>,
    created_rx: Res<OutputReceiver<CreateOutput>>,
) {
    if let Ok(created_result) = created_rx.0.try_recv() {
        if let Some(created) = created_result {
            bag_output.create_output = Some(created);
        }
        bag_input.create_input.loading = false;
    }
}

pub struct CreateBagPlugin;

impl Plugin for CreateBagPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_in_out_channels::<CreateBagRequest, CreateOutput>)
            .add_system(request_handler::<CreateBagRequest, CreateInput, CreateOutput>)
            .add_system(handle_create_response);
    }
}
