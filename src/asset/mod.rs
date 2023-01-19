use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};
use sugarfunge_api_types::asset::*;

use crate::prelude::*;

pub mod create;
pub mod info;
pub mod mint;
pub mod update_metadata;

#[derive(Resource, Debug, Default, Eq, PartialEq)]
pub enum AssetActions {
    #[default]
    CreateAsset,
    AssetInfo,
    UpdateAssetMetadata,
    AssetMint,
    AssetBurn,
    AssetBalance,
    AssetBalances,
    AssetTransferFrom,
}

#[derive(Resource, Debug, Default, Clone)]
pub struct AssetInputData {
    create_input: create::CreateAssetInputData,
    info_input: info::AssetInfoInputData,
    update_metadata_input: update_metadata::UpdateAssetMetadataInputData,
    mint_input: mint::AssetMintInputData,
    // burn_input: burn::AssetBurnInputData,
    // balance_input: balance::AssetBalanceInputData,
    // balances_input: balances::AssetBalancesInputData,
    // transfer_from_input: transfer_from::AssetTransferFromInputData,
}

#[derive(Resource, Default, Debug)]
pub struct AssetOutputData {
    create_output: Option<CreateOutput>,
    info_output: Option<AssetInfoOutput>,
    update_metadata_output: Option<UpdateMetadataOutput>,
    mint_output: Option<MintOutput>,
    burn_output: Option<BurnOutput>,
    balance_output: Option<AssetBalanceOutput>,
    balances_output: Option<AssetBalancesOutput>,
    transfer_from_output: Option<TransferFromOutput>,
}

pub fn asset_ui(
    mut ctx: ResMut<EguiContext>,
    mut asset_actions: ResMut<AssetActions>,
    mut asset_input: ResMut<AssetInputData>,
    asset_output: Res<AssetOutputData>,
    create_tx: Res<InputSender<create::CreateAssetRequest>>,
    info_tx: Res<InputSender<info::AssetInfoRequest>>,
    update_metadata_tx: Res<InputSender<update_metadata::UpdateMetadataRequest>>,
    mint_tx: Res<InputSender<mint::AssetMintRequest>>,
    // burn_tx: Res<InputSender<AssetBurnRequest>>,
    // balance_tx: Res<InputSender<AssetBalanceRequest>>,
    // balances_tx: Res<InputSender<AssetBalancesRequest>>,
    // transfer_from_tx: Res<InputSender<AssetTransferFromRequest>>,
) {
    egui::Window::new("Asset").show(&mut ctx.ctx_mut(), |ui| {
        ui.horizontal(|ui| {
            ui.selectable_value(&mut *asset_actions, AssetActions::CreateAsset, "Create");
            ui.selectable_value(&mut *asset_actions, AssetActions::AssetInfo, "Info");
            ui.selectable_value(
                &mut *asset_actions,
                AssetActions::UpdateAssetMetadata,
                "Update Metadata",
            );
            ui.selectable_value(&mut *asset_actions, AssetActions::AssetMint, "Mint");
        });
        ui.horizontal(|ui| {
            ui.selectable_value(&mut *asset_actions, AssetActions::AssetBurn, "Burn");
            ui.selectable_value(&mut *asset_actions, AssetActions::AssetBalance, "Balance");
            ui.selectable_value(&mut *asset_actions, AssetActions::AssetBalances, "Balances");
            ui.selectable_value(
                &mut *asset_actions,
                AssetActions::AssetTransferFrom,
                "Transfer From",
            );
        });
        ui.separator();
        match &*asset_actions {
            AssetActions::CreateAsset => {
                create::create_asset_ui(ui, &mut asset_input, &create_tx, &asset_output);
            }
            AssetActions::AssetInfo => {
                info::asset_info_ui(ui, &mut asset_input, &info_tx, &asset_output);
            }
            AssetActions::UpdateAssetMetadata => {
                update_metadata::update_asset_metadata_ui(
                    ui,
                    &mut asset_input,
                    &update_metadata_tx,
                    &asset_output,
                );
            }
            AssetActions::AssetMint => {
                mint::asset_mint_ui(ui, &mut asset_input, &mint_tx, &asset_output);
            }
            AssetActions::AssetBurn => {
                // burn::asset_burn_ui(ui, &mut asset_input, &burn_tx, &asset_output);
            }
            AssetActions::AssetBalance => {
                // balance::asset_balance_ui(ui, &mut asset_input, &balance_tx, &asset_output);
            }
            AssetActions::AssetBalances => {
                // balances::asset_balances_ui(ui, &mut asset_input, &balances_tx, &asset_output);
            }
            AssetActions::AssetTransferFrom => {
                // transfer_from::asset_transfer_from_ui(
                //     ui,
                //     &mut asset_input,
                //     &transfer_from_tx,
                //     &asset_output,
                // );
            }
        }
    });
}

pub struct AssetPlugin;

impl Plugin for AssetPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AssetActions>()
            .init_resource::<AssetInputData>()
            .init_resource::<AssetOutputData>()
            .add_system(asset_ui)
            .add_plugin(create::CreateAssetPlugin)
            .add_plugin(info::AssetInfoPlugin)
            .add_plugin(update_metadata::UpdateAssetMetadataPlugin)
            .add_plugin(mint::AssetMintPlugin);
    }
}
