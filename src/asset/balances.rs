use bevy::prelude::*;
use bevy_egui::egui;
use sugarfunge_api_types::{
    asset::{AssetBalancesInput, AssetBalancesOutput},
    primitives::{Account, ClassId},
};

use crate::{
    prelude::*,
    util::{request_handler, setup_in_out_channels},
};

use super::{AssetInputData, AssetOutputData};

#[derive(Debug)]
pub struct AssetBalancesRequest {
    pub input: AssetBalancesInput,
}

impl Request<AssetBalancesInput> for AssetBalancesRequest {
    fn endpoint(&self) -> &str {
        "asset/balances"
    }

    fn input(&self) -> Option<AssetBalancesInput> {
        Some(AssetBalancesInput {
            class_id: self.input.class_id.clone(),
            account: self.input.account.clone(),
        })
    }
}

#[derive(Resource, Debug, Clone)]
pub struct AssetBalancesInputData {
    pub class_id: ClassId,
    pub class_id_enabled: bool,
    pub account: Account,
    pub loading: bool,
}

impl Default for AssetBalancesInputData {
    fn default() -> Self {
        Self {
            class_id: ClassId::from(0),
            class_id_enabled: false,
            account: Account::from("".to_string()),
            loading: false,
        }
    }
}

pub fn asset_balances_ui(
    ui: &mut egui::Ui,
    asset_input: &mut ResMut<AssetInputData>,
    balances_tx: &Res<InputSender<AssetBalancesRequest>>,
    asset_output: &Res<AssetOutputData>,
) {
    ui.label("Asset Balances");
    ui.separator();
    ui.label("Account");
    ui.text_edit_singleline(&mut *asset_input.balances_input.account);
    ui.checkbox(
        &mut asset_input.balances_input.class_id_enabled,
        "Enable Class ID",
    );
    if asset_input.balances_input.class_id_enabled {
        ui.label("Class ID");
        ui.add(egui::DragValue::new::<u64>(&mut *asset_input.balances_input.class_id).speed(0.1));
    }
    ui.separator();
    if asset_input.balances_input.loading {
        ui.separator();
        ui.add(egui::Spinner::default());
    } else {
        if ui.button("Get Balances").clicked() {
            balances_tx
                .send(AssetBalancesRequest {
                    input: AssetBalancesInput {
                        class_id: if asset_input.balances_input.class_id_enabled {
                            Some(asset_input.balances_input.class_id)
                        } else {
                            None
                        },
                        account: asset_input.balances_input.account.clone(),
                    },
                })
                .unwrap();
            asset_input.balances_input.loading = true;
        }
    }
    ui.separator();
    if let Some(balances_output) = &asset_output.balances_output {
        ui.label("Balances");
        ui.separator();
        for (i, balance) in balances_output.balances.iter().enumerate() {
            ui.label(format!("Amount [{}]", i + 1));
            ui.text_edit_singleline(&mut format!("{:?}", balance.amount));
            ui.separator();
        }
    }
}

pub fn handle_balances_response(
    mut asset_output: ResMut<AssetOutputData>,
    mut asset_input: ResMut<AssetInputData>,
    balances_rx: Res<OutputReceiver<AssetBalancesOutput>>,
) {
    if let Ok(balances_result) = balances_rx.0.try_recv() {
        if let Some(balances) = balances_result {
            asset_output.balances_output = Some(balances);
        }
        asset_input.balances_input.loading = false;
    }
}

pub struct AssetBalancesPlugin;

impl Plugin for AssetBalancesPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_in_out_channels::<AssetBalancesRequest, AssetBalancesOutput>)
            .add_system(
                request_handler::<AssetBalancesRequest, AssetBalancesInput, AssetBalancesOutput>,
            )
            .add_system(handle_balances_response);
    }
}
