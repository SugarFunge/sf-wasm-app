use bevy::prelude::*;
use bevy_egui::egui;
use sugarfunge_api_types::asset::*;

pub mod create;
pub mod info;

#[derive(Resource, Default)]
pub struct ClassUi {
    pub actions: ClassActions,
    pub data: ClassData,
    pub channels: ClassChannels,
}

#[derive(Resource, Debug, Default, Eq, PartialEq)]
pub enum ClassActions {
    #[default]
    CreateClass,
    ClassInfo,
}

#[derive(Resource, Debug, Default, Clone)]
pub struct ClassInputData {
    create: create::CreateClassInputData,
    info: info::ClassInfoInputData,
}

#[derive(Resource, Default, Debug)]
pub struct ClassOutputData {
    create: Option<CreateClassOutput>,
    info: Option<ClassInfoOutput>,
}

#[derive(Resource, Default)]
pub struct ClassData {
    pub input: ClassInputData,
    pub output: ClassOutputData,
}

#[derive(Resource, Default)]
pub struct ClassChannels {
    create: create::CreateClassChannel,
    info: info::ClassInfoChannel,
}

pub fn class_ui(ui: &mut egui::Ui, class: &mut ResMut<ClassUi>) {
    ui.horizontal(|ui| {
        ui.selectable_value(&mut class.actions, ClassActions::CreateClass, "Create");
        ui.selectable_value(&mut class.actions, ClassActions::ClassInfo, "Info");
    });
    ui.separator();
    match &class.actions {
        ClassActions::CreateClass => {
            create::create_class_ui(ui, class);
        }
        ClassActions::ClassInfo => {
            info::class_info_ui(ui, class);
        }
    }
}

pub struct ClassPlugin;

impl Plugin for ClassPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ClassUi>()
            .add_plugin(create::CreateClassPlugin)
            .add_plugin(info::ClassInfoPlugin);
    }
}
