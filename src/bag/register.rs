use bevy::prelude::*;
use bevy_egui::egui;
use crossbeam::channel;
use sugarfunge_api_types::{
    bag::{RegisterInput, RegisterOutput},
    primitives::{ClassId, Seed},
};

use crate::{prelude::*, util::request_handler};

use super::BagUi;

#[derive(Debug)]
pub struct RegisterBagRequest {
    pub input: RegisterInput,
}

impl Request<RegisterInput> for RegisterBagRequest {
    fn endpoint(&self) -> &str {
        "bag/register"
    }

    fn input(&self) -> Option<RegisterInput> {
        Some(RegisterInput {
            seed: self.input.seed.clone(),
            class_id: self.input.class_id.clone(),
            metadata: self.input.metadata.clone(),
        })
    }
}

#[derive(Resource)]
pub struct RegisterBagChannel {
    pub input_tx: InputSender<RegisterBagRequest>,
    pub input_rx: InputReceiver<RegisterBagRequest>,
    pub output_tx: OutputSender<RegisterOutput>,
    pub output_rx: OutputReceiver<RegisterOutput>,
}

impl Default for RegisterBagChannel {
    fn default() -> Self {
        let (input_tx, input_rx) = channel::unbounded::<RegisterBagRequest>();
        let (output_tx, output_rx) = channel::unbounded::<Option<RegisterOutput>>();
        Self {
            input_tx: InputSender(input_tx),
            input_rx: InputReceiver(input_rx),
            output_tx: OutputSender(output_tx),
            output_rx: OutputReceiver(output_rx),
        }
    }
}

#[derive(Resource, Debug, Clone)]
pub struct RegisterBagInputData {
    pub seed: Seed,
    pub class_id: ClassId,
    pub metadata: String,
    pub loading: bool,
}

impl Default for RegisterBagInputData {
    fn default() -> Self {
        Self {
            seed: Seed::from("".to_string()),
            class_id: ClassId::from(0),
            metadata: "".to_string(),
            loading: false,
        }
    }
}

pub fn register_bag_ui(ui: &mut egui::Ui, bag: &mut ResMut<BagUi>) {
    ui.label("Register Bag");
    ui.separator();
    ui.label("Seed");
    ui.text_edit_singleline(&mut *bag.data.input.register.seed);
    ui.label("Class ID");
    ui.add(egui::DragValue::new::<u64>(&mut *bag.data.input.register.class_id).speed(0.1));
    ui.label("Metadata");
    ui.text_edit_multiline(&mut bag.data.input.register.metadata);
    if bag.data.input.register.loading {
        ui.separator();
        ui.add(egui::Spinner::default());
    } else {
        if ui.button("Register").clicked() {
            bag.channels
                .register
                .input_tx
                .0
                .send(RegisterBagRequest {
                    input: RegisterInput {
                        seed: bag.data.input.register.seed.clone(),
                        class_id: bag.data.input.register.class_id,
                        metadata: serde_json::from_str(&bag.data.input.register.metadata).unwrap(),
                    },
                })
                .unwrap();
            bag.data.input.register.loading = true;
        }
    }
    if let Some(output) = &bag.data.output.register {
        ui.separator();
        ui.label("Who");
        ui.text_edit_singleline(&mut output.who.as_str());
        ui.label("Class ID");
        ui.text_edit_singleline(&mut u64::from(output.class_id).to_string());
    }
}

pub fn handle_register_response(mut bag: ResMut<BagUi>, tokio_runtime: Res<TokioRuntime>) {
    if let Ok(registered_result) = bag.channels.register.output_rx.0.try_recv() {
        if let Some(registered) = registered_result {
            bag.data.output.register = Some(registered);
        }
        bag.data.input.register.loading = false;
    }

    request_handler::<RegisterBagRequest, RegisterInput, RegisterOutput>(
        tokio_runtime.runtime.clone(),
        bag.channels.register.input_rx.clone(),
        bag.channels.register.output_tx.clone(),
    );
}

pub struct RegisterBagPlugin;

impl Plugin for RegisterBagPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(handle_register_response);
    }
}
