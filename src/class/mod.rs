use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};
use sugarfunge_api_types::asset::*;

use crate::prelude::*;

pub mod create;
pub mod info;

#[derive(Resource, Debug, Default, Eq, PartialEq)]
pub enum ClassActions {
    #[default]
    CreateClass,
    ClassInfo,
}

#[derive(Resource, Debug, Default, Clone)]
pub struct ClassInputData {
    create_input: create::CreateClassInputData,
    info_input: info::ClassInfoInputData,
}

#[derive(Resource, Default, Debug)]
pub struct ClassOutputData {
    create_output: Option<CreateClassOutput>,
    info_output: Option<ClassInfoOutput>,
}

pub fn class_ui(
    mut ctx: ResMut<EguiContext>,
    mut class_actions: ResMut<ClassActions>,
    mut class_input: ResMut<ClassInputData>,
    class_output: Res<ClassOutputData>,
    create_tx: Res<InputSender<create::CreateClassRequest>>,
    info_tx: Res<InputSender<info::ClassInfoRequest>>,
) {
    egui::Window::new("Class").show(&mut ctx.ctx_mut(), |ui| {
        ui.horizontal(|ui| {
            ui.selectable_value(&mut *class_actions, ClassActions::CreateClass, "Create");
            ui.selectable_value(&mut *class_actions, ClassActions::ClassInfo, "Info");
        });
        ui.separator();
        match &*class_actions {
            ClassActions::CreateClass => {
                create::create_class_ui(ui, &mut class_input, &create_tx, &class_output);
            }
            ClassActions::ClassInfo => {
                info::class_info_ui(ui, &mut class_input, &info_tx, &class_output);
            }
        }
    });
}

pub struct ClassPlugin;

impl Plugin for ClassPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ClassActions>()
            .init_resource::<ClassInputData>()
            .init_resource::<ClassOutputData>()
            .add_plugin(create::CreateClassPlugin)
            .add_plugin(info::ClassInfoPlugin)
            .add_system(class_ui);
    }
}
