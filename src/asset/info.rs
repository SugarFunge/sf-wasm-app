use bevy::prelude::*;
use bevy_egui::egui;
use sugarfunge_api_types::{
    asset::{AssetInfoInput, AssetInfoOutput},
    primitives::{AssetId, ClassId},
};

use crate::{
    prelude::*,
    util::{request_handler, setup_in_out_channels},
};

use super::{AssetInputData, AssetOutputData};

#[derive(Debug)]
pub struct AssetInfoRequest {
    pub input: AssetInfoInput,
}

impl Request<AssetInfoInput> for AssetInfoRequest {
    fn endpoint(&self) -> &str {
        "asset/info"
    }

    fn input(&self) -> Option<AssetInfoInput> {
        Some(AssetInfoInput {
            asset_id: self.input.asset_id.clone(),
            class_id: self.input.class_id.clone(),
        })
    }
}

#[derive(Resource, Debug, Default, Clone)]
pub struct AssetInfoInputData {
    pub asset_id: u64,
    pub class_id: u64,
    pub loading: bool,
}

pub fn asset_info_ui(
    ui: &mut egui::Ui,
    asset_input: &mut ResMut<AssetInputData>,
    info_tx: &Res<InputSender<AssetInfoRequest>>,
    asset_output: &Res<AssetOutputData>,
) {
    ui.label("Asset Info");
    ui.separator();
    ui.label("Class ID");
    ui.add(egui::DragValue::new::<u64>(&mut asset_input.info_input.class_id).speed(0.1));
    ui.label("Asset ID");
    ui.add(egui::DragValue::new::<u64>(&mut asset_input.info_input.asset_id).speed(0.1));
    if asset_input.update_metadata_input.loading {
        ui.separator();
        ui.add(egui::Spinner::default());
    } else {
        if ui.button("Get Info").clicked() {
            info_tx
                .0
                .send(AssetInfoRequest {
                    input: AssetInfoInput {
                        asset_id: AssetId::from(asset_input.info_input.asset_id),
                        class_id: ClassId::from(asset_input.info_input.class_id),
                    },
                })
                .unwrap();
        }
        asset_input.info_input.loading = true;
    }
    if let Some(output) = &asset_output.info_output {
        ui.separator();
        if let Some(info) = &output.info {
            ui.label("Class ID");
            ui.text_edit_singleline(&mut format!("{:?}", info.class_id));
            ui.label("Asset ID");
            ui.text_edit_singleline(&mut format!("{:?}", info.asset_id));
            ui.label("Metadata");
            ui.text_edit_singleline(&mut info.metadata.to_string());
        } else {
            ui.label("No asset info found");
        }
    }
}

pub fn handle_info_response(
    mut asset_output: ResMut<AssetOutputData>,
    mut asset_input: ResMut<AssetInputData>,
    info_rx: Res<OutputReceiver<AssetInfoOutput>>,
) {
    if let Ok(info_result) = info_rx.0.try_recv() {
        if let Some(info) = info_result {
            asset_output.info_output = Some(info);
        }
        asset_input.info_input.loading = false;
    }
}

pub struct AssetInfoPlugin;

impl Plugin for AssetInfoPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_in_out_channels::<AssetInfoRequest, AssetInfoOutput>)
            .add_system(request_handler::<AssetInfoRequest, AssetInfoInput, AssetInfoOutput>)
            .add_system(handle_info_response);
    }
}
