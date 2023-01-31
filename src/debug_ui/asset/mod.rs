use bevy::prelude::*;
use bevy_egui::egui;
use sugarfunge_api_types::asset::*;

pub mod balance;
pub mod balances;
pub mod burn;
pub mod create;
pub mod info;
pub mod mint;
pub mod transfer_from;
pub mod update_metadata;

#[derive(Resource, Default)]
pub struct AssetUi {
    pub actions: AssetActions,
    pub data: AssetData,
    pub channels: AssetChannels,
}

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
    create: create::CreateAssetInputData,
    info: info::AssetInfoInputData,
    update_metadata: update_metadata::UpdateAssetMetadataInputData,
    mint: mint::AssetMintInputData,
    burn: burn::AssetBurnInputData,
    balance: balance::AssetBalanceInputData,
    balances: balances::AssetBalancesInputData,
    transfer_from: transfer_from::AssetTransferFromInputData,
}

#[derive(Resource, Default, Debug)]
pub struct AssetOutputData {
    create: Option<CreateOutput>,
    info: Option<AssetInfoOutput>,
    update_metadata: Option<UpdateMetadataOutput>,
    mint: Option<MintOutput>,
    burn: Option<BurnOutput>,
    balance: Option<AssetBalanceOutput>,
    balances: Option<AssetBalancesOutput>,
    transfer_from: Option<TransferFromOutput>,
}

#[derive(Resource, Default)]
pub struct AssetData {
    pub input: AssetInputData,
    pub output: AssetOutputData,
}

#[derive(Resource, Default)]
pub struct AssetChannels {
    create: create::CreateAssetChannel,
    info: info::AssetInfoChannel,
    update_metadata: update_metadata::UpdateMetadataChannel,
    mint: mint::AssetMintChannel,
    burn: burn::AssetBurnChannel,
    balance: balance::AssetBalanceChannel,
    balances: balances::AssetBalancesChannel,
    transfer_from: transfer_from::AssetTransferFromChannel,
}

pub fn asset_ui(ui: &mut egui::Ui, asset: &mut ResMut<AssetUi>) {
    ui.horizontal(|ui| {
        ui.selectable_value(&mut asset.actions, AssetActions::CreateAsset, "Create");
        ui.selectable_value(&mut asset.actions, AssetActions::AssetInfo, "Info");
        ui.selectable_value(
            &mut asset.actions,
            AssetActions::UpdateAssetMetadata,
            "Update Metadata",
        );
        ui.selectable_value(&mut asset.actions, AssetActions::AssetMint, "Mint");
    });
    ui.horizontal(|ui| {
        ui.selectable_value(&mut asset.actions, AssetActions::AssetBurn, "Burn");
        ui.selectable_value(&mut asset.actions, AssetActions::AssetBalance, "Balance");
        ui.selectable_value(&mut asset.actions, AssetActions::AssetBalances, "Balances");
        ui.selectable_value(
            &mut asset.actions,
            AssetActions::AssetTransferFrom,
            "Transfer From",
        );
    });
    ui.separator();
    match asset.actions {
        AssetActions::CreateAsset => {
            create::create_asset_ui(ui, asset);
        }
        AssetActions::AssetInfo => {
            info::asset_info_ui(ui, asset);
        }
        AssetActions::UpdateAssetMetadata => {
            update_metadata::update_asset_metadata_ui(ui, asset);
        }
        AssetActions::AssetMint => {
            mint::asset_mint_ui(ui, asset);
        }
        AssetActions::AssetBurn => {
            burn::asset_burn_ui(ui, asset);
        }
        AssetActions::AssetBalance => {
            balance::asset_balance_ui(ui, asset);
        }
        AssetActions::AssetBalances => {
            balances::asset_balances_ui(ui, asset);
        }
        AssetActions::AssetTransferFrom => {
            transfer_from::asset_transfer_from_ui(ui, asset);
        }
    }
}

pub struct AssetPlugin;

impl Plugin for AssetPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AssetUi>()
            .add_plugin(create::CreateAssetPlugin)
            .add_plugin(info::AssetInfoPlugin)
            .add_plugin(update_metadata::UpdateAssetMetadataPlugin)
            .add_plugin(mint::AssetMintPlugin)
            .add_plugin(burn::AssetBurnPlugin)
            .add_plugin(balance::AssetBalancePlugin)
            .add_plugin(balances::AssetBalancesPlugin)
            .add_plugin(transfer_from::AssetTransferFromPlugin);
    }
}
