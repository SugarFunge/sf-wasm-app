use bevy::prelude::*;
use bevy_egui::egui;
use sugarfunge_api_types::account::*;

pub mod balance;
pub mod create;
pub mod exists;
pub mod fund;
pub mod seeded;

#[derive(Resource, Default)]
pub struct AccountUi {
    pub actions: AccountActions,
    pub data: AccountData,
    pub channels: AccountChannels,
}

#[derive(Resource, Debug, Default, Eq, PartialEq)]
pub enum AccountActions {
    #[default]
    CreateAccount,
    FundAccount,
    GetAccountBalance,
    GetSeededAccount,
    GetAccountExists,
}

#[derive(Resource, Debug, Default, Clone)]
pub struct AccountInputData {
    create: create::CreateAccountInputData,
    fund: fund::FundAccountInputData,
    exists: exists::AccountExistsInputData,
    seeded: seeded::SeededAccountInputData,
    balance: balance::AccountBalanceInputData,
}

#[derive(Resource, Default, Debug)]
pub struct AccountOutputData {
    create: Option<CreateAccountOutput>,
    fund: Option<FundAccountOutput>,
    exists: Option<AccountExistsOutput>,
    seeded: Option<SeededAccountOutput>,
    balance: Option<AccountBalanceOutput>,
}

#[derive(Resource, Default)]
pub struct AccountData {
    input: AccountInputData,
    output: AccountOutputData,
}

#[derive(Resource, Default)]
pub struct AccountChannels {
    create: create::CreateAccountChannel,
    fund: fund::FundAccountChannel,
    exists: exists::AccountExistsChannel,
    seeded: seeded::SeededAccountChannel,
    balance: balance::AccountBalanceChannel,
}

pub fn account_ui(ui: &mut egui::Ui, account: &mut ResMut<AccountUi>) {
    ui.horizontal(|ui| {
        ui.selectable_value(
            &mut account.actions,
            AccountActions::CreateAccount,
            "Create",
        );
        ui.selectable_value(&mut account.actions, AccountActions::FundAccount, "Fund");
        ui.selectable_value(
            &mut account.actions,
            AccountActions::GetAccountBalance,
            "Balance",
        );
        ui.selectable_value(
            &mut account.actions,
            AccountActions::GetSeededAccount,
            "Seeded",
        );
        ui.selectable_value(
            &mut account.actions,
            AccountActions::GetAccountExists,
            "Exists",
        );
    });
    ui.separator();
    match &account.actions {
        AccountActions::CreateAccount => {
            create::create_account_ui(ui, account);
        }
        AccountActions::FundAccount => {
            fund::account_fund_ui(ui, account);
        }
        AccountActions::GetAccountBalance => {
            balance::account_balance_ui(ui, account);
        }
        AccountActions::GetSeededAccount => {
            seeded::seeded_account_ui(ui, account);
        }
        AccountActions::GetAccountExists => {
            exists::account_exists_ui(ui, account);
        }
    }
}

pub struct AccountPlugin;

impl Plugin for AccountPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AccountUi>()
            .add_plugin(create::AccountCreatePlugin)
            .add_plugin(fund::AccountFundPlugin)
            .add_plugin(exists::AccountExistsPlugin)
            .add_plugin(seeded::SeededAccountPlugin)
            .add_plugin(balance::AccountBalancePlugin);
    }
}
