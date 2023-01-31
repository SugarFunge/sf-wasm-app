use bevy::prelude::*;
use bevy_egui::egui;
use crossbeam::channel;
use sugarfunge_api_types::{
    bag::{CreateInput, CreateOutput},
    primitives::{Account, Balance, ClassId, Seed},
};

use crate::{
    prelude::*,
    util::{request_handler, vec_u64_input_ui},
};

use super::BagUi;

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

#[derive(Resource)]
pub struct CreateBagChannel {
    pub input_tx: InputSender<CreateBagRequest>,
    pub input_rx: InputReceiver<CreateBagRequest>,
    pub output_tx: OutputSender<CreateOutput>,
    pub output_rx: OutputReceiver<CreateOutput>,
}

impl Default for CreateBagChannel {
    fn default() -> Self {
        let (input_tx, input_rx) = channel::unbounded::<CreateBagRequest>();
        let (output_tx, output_rx) = channel::unbounded::<Option<CreateOutput>>();
        Self {
            input_tx: InputSender(input_tx),
            input_rx: InputReceiver(input_rx),
            output_tx: OutputSender(output_tx),
            output_rx: OutputReceiver(output_rx),
        }
    }
}

#[derive(Resource, Debug, Clone)]
pub struct CreateBagInputData {
    pub seed: Seed,
    pub class_id: ClassId,
    pub owners: Vec<Account>,
    pub shares: Vec<u64>,
    pub loading: bool,
}

impl Default for CreateBagInputData {
    fn default() -> Self {
        Self {
            seed: Seed::from("".to_string()),
            class_id: ClassId::from(0),
            owners: vec![],
            shares: vec![],
            loading: false,
        }
    }
}

pub fn create_bag_ui(ui: &mut egui::Ui, bag: &mut ResMut<BagUi>) {
    ui.label("Create Bag");
    ui.separator();
    ui.label("Seed");
    ui.text_edit_singleline(&mut *bag.data.input.create.seed);
    ui.label("Class ID");
    ui.add(egui::DragValue::new::<u64>(&mut bag.data.input.create.class_id).speed(0.1));
    ui.label("Owners");
    if ui.button("Add Owner").clicked() {
        bag.data
            .input
            .create
            .owners
            .push(Account::from("".to_string()));
    }
    let owners = bag.data.input.create.owners.clone();
    let mut owner_remove_index: Option<usize> = None;
    for (i, _) in owners.iter().enumerate() {
        ui.horizontal(|ui| {
            ui.text_edit_singleline(&mut *bag.data.input.create.owners[i]);
            if ui.button("Remove").clicked() {
                owner_remove_index = Some(i);
            }
        });
    }
    if let Some(index) = owner_remove_index {
        bag.data.input.create.owners.remove(index);
    }
    ui.label("Shares");
    ui.label("The Shares are represented in 10^18 units.");
    vec_u64_input_ui(ui, &mut bag.data.input.create.shares);
    if bag.data.input.create.loading {
        ui.separator();
        ui.add(egui::Spinner::default());
    } else {
        if ui.button("Create").clicked() {
            let mut shares_input = Vec::new();
            for share in bag.data.input.create.shares.iter() {
                shares_input.push(Balance::from((*share as u128) * (u128::pow(10, 18))));
            }
            bag.channels
                .create
                .input_tx
                .0
                .send(CreateBagRequest {
                    input: CreateInput {
                        seed: bag.data.input.create.seed.clone(),
                        class_id: bag.data.input.create.class_id,
                        owners: bag.data.input.create.owners.clone(),

                        shares: shares_input,
                    },
                })
                .unwrap();
            bag.data.input.create.loading = true;
        }
    }
    if let Some(output) = &bag.data.output.create {
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

pub fn handle_create_response(mut bag: ResMut<BagUi>, tokio_runtime: Res<TokioRuntime>) {
    if let Ok(created_result) = bag.channels.create.output_rx.0.try_recv() {
        if let Some(created) = created_result {
            bag.data.output.create = Some(created);
        }
        bag.data.input.create.loading = false;
    }

    request_handler::<CreateBagRequest, CreateInput, CreateOutput>(
        tokio_runtime.runtime.clone(),
        bag.channels.create.input_rx.clone(),
        bag.channels.create.output_tx.clone(),
    );
}

pub struct CreateBagPlugin;

impl Plugin for CreateBagPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(handle_create_response);
    }
}
