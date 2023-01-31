use bevy::prelude::*;
use bevy_egui::egui;
use crossbeam::channel;
use sugarfunge_api_types::{
    asset::{ClassInfoInput, ClassInfoOutput},
    primitives::ClassId,
};

use crate::{prelude::*, util::request_handler};

use super::ClassUi;

#[derive(Debug)]
pub struct ClassInfoRequest {
    pub input: ClassInfoInput,
}

impl Request<ClassInfoInput> for ClassInfoRequest {
    fn endpoint(&self) -> &str {
        "asset/class_info"
    }

    fn input(&self) -> Option<ClassInfoInput> {
        Some(ClassInfoInput {
            class_id: self.input.class_id.clone(),
        })
    }
}

#[derive(Resource)]
pub struct ClassInfoChannel {
    pub input_tx: InputSender<ClassInfoRequest>,
    pub input_rx: InputReceiver<ClassInfoRequest>,
    pub output_tx: OutputSender<ClassInfoOutput>,
    pub output_rx: OutputReceiver<ClassInfoOutput>,
}

impl Default for ClassInfoChannel {
    fn default() -> Self {
        let (input_tx, input_rx) = channel::unbounded::<ClassInfoRequest>();
        let (output_tx, output_rx) = channel::unbounded::<Option<ClassInfoOutput>>();
        Self {
            input_tx: InputSender(input_tx),
            input_rx: InputReceiver(input_rx),
            output_tx: OutputSender(output_tx),
            output_rx: OutputReceiver(output_rx),
        }
    }
}

#[derive(Resource, Debug, Default, Clone)]
pub struct ClassInfoInputData {
    pub class_id: u64,
    pub loading: bool,
}

pub fn class_info_ui(ui: &mut egui::Ui, class: &mut ResMut<ClassUi>) {
    ui.label("Class Info");
    ui.separator();
    ui.label("Class ID");
    ui.add(egui::DragValue::new::<u64>(&mut class.data.input.info.class_id).speed(0.1));
    if class.data.input.info.loading {
        ui.separator();
        ui.add(egui::Spinner::default());
    } else {
        if ui.button("Get Info").clicked() {
            class
                .channels
                .info
                .input_tx
                .0
                .send(ClassInfoRequest {
                    input: ClassInfoInput {
                        class_id: ClassId::from(class.data.input.info.class_id),
                    },
                })
                .unwrap();
            class.data.input.info.loading = true;
        }
    }
    if let Some(output) = &class.data.output.info {
        ui.separator();
        if let Some(info) = &output.info {
            ui.label("Class ID");
            ui.text_edit_singleline(&mut u64::from(info.class_id).to_string());
            ui.label("Metadata");
            ui.text_edit_multiline(&mut info.metadata.to_string());
            ui.label("Owner");
            ui.text_edit_singleline(&mut info.owner.as_str());
        } else {
            ui.label("No class info found");
        }
    }
}

pub fn handle_info_response(mut class: ResMut<ClassUi>, tokio_runtime: Res<TokioRuntime>) {
    if let Ok(info_result) = class.channels.info.output_rx.0.try_recv() {
        if let Some(info) = info_result {
            class.data.output.info = Some(info);
        }
        class.data.input.info.loading = false;
    }

    request_handler::<ClassInfoRequest, ClassInfoInput, ClassInfoOutput>(
        tokio_runtime.runtime.clone(),
        class.channels.info.input_rx.clone(),
        class.channels.info.output_tx.clone(),
    );
}

pub struct ClassInfoPlugin;

impl Plugin for ClassInfoPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(handle_info_response);
    }
}
