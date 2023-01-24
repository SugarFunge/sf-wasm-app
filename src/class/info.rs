use bevy::prelude::*;
use bevy_egui::egui;
use sugarfunge_api_types::{
    asset::{ClassInfoInput, ClassInfoOutput},
    primitives::ClassId,
};

use crate::{
    prelude::*,
    util::{request_handler, setup_in_out_channels},
};

use super::{ClassInputData, ClassOutputData};

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

#[derive(Resource, Debug, Default, Clone)]
pub struct ClassInfoInputData {
    pub class_id: u64,
    pub loading: bool,
}

pub fn class_info_ui(
    ui: &mut egui::Ui,
    class_input: &mut ResMut<ClassInputData>,
    info_tx: &Res<InputSender<ClassInfoRequest>>,
    class_output: &Res<ClassOutputData>,
) {
    ui.label("Class Info");
    ui.separator();
    ui.label("Class ID");
    ui.add(egui::DragValue::new::<u64>(&mut class_input.info_input.class_id).speed(0.1));
    if class_input.info_input.loading {
        ui.separator();
        ui.add(egui::Spinner::default());
    } else {
        if ui.button("Get Info").clicked() {
            info_tx
                .0
                .send(ClassInfoRequest {
                    input: ClassInfoInput {
                        class_id: ClassId::from(class_input.info_input.class_id),
                    },
                })
                .unwrap();
            class_input.info_input.loading = true;
        }
    }
    if let Some(output) = &class_output.info_output {
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

pub fn handle_info_response(
    mut class_output: ResMut<ClassOutputData>,
    mut class_input: ResMut<ClassInputData>,
    info_rx: Res<OutputReceiver<ClassInfoOutput>>,
) {
    if let Ok(info_result) = info_rx.0.try_recv() {
        if let Some(info) = info_result {
            class_output.info_output = Some(info);
        }
        class_input.info_input.loading = false;
    }
}

pub struct ClassInfoPlugin;

impl Plugin for ClassInfoPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_in_out_channels::<ClassInfoRequest, ClassInfoOutput>)
            .add_system(request_handler::<ClassInfoRequest, ClassInfoInput, ClassInfoOutput>)
            .add_system(handle_info_response);
    }
}
