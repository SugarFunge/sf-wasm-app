use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};
use sugarfunge_api_types::account::*;

use crate::prelude::*;

pub mod balance;
pub mod create;
pub mod exists;
pub mod fund;
pub mod seeded;

#[derive(Resource, Debug, Default, Eq, PartialEq)]
pub enum AccountActions {
    GetSeededAccount,
    GetAccountExists,
    #[default]
    CreateAccount,
    FundAccount,
    GetAccountBalance,
}

#[derive(Resource, Debug, Default, Clone)]
pub struct AccountInputData {
    create_input: create::CreateAccountInputData,
    fund_input: fund::FundAccountInputData,
    exists_input: exists::AccountExistsInputData,
    seeded_input: seeded::SeededAccountInputData,
    balance_input: balance::AccountBalanceInputData,
}

#[derive(Resource, Default, Debug)]
pub struct AccountOutputData {
    create_output: Option<CreateAccountOutput>,
    fund_output: Option<FundAccountOutput>,
    exists_output: Option<AccountExistsOutput>,
    seeded_output: Option<SeededAccountOutput>,
    balance_output: Option<AccountBalanceOutput>,
}

pub fn account_ui(
    mut ctx: ResMut<EguiContext>,
    mut account_actions: ResMut<AccountActions>,
    mut account_input: ResMut<AccountInputData>,
    account_output: Res<AccountOutputData>,
    created_tx: Res<InputSender<create::CreateAccountRequest>>,
    funded_tx: Res<InputSender<fund::FundAccountRequest>>,
    exists_tx: Res<InputSender<exists::AccountExistsRequest>>,
    seeded_tx: Res<InputSender<seeded::SeededAccountRequest>>,
    balance_tx: Res<InputSender<balance::AccountBalanceRequest>>,
) {
    egui::Window::new("Account").show(&mut ctx.ctx_mut(), |ui| {
        ui.horizontal(|ui| {
            ui.selectable_value(
                &mut *account_actions,
                AccountActions::GetSeededAccount,
                "Seeded",
            );
            ui.selectable_value(
                &mut *account_actions,
                AccountActions::GetAccountExists,
                "Exists",
            );
            ui.selectable_value(
                &mut *account_actions,
                AccountActions::CreateAccount,
                "Create",
            );
            ui.selectable_value(&mut *account_actions, AccountActions::FundAccount, "Fund");
            ui.selectable_value(
                &mut *account_actions,
                AccountActions::GetAccountBalance,
                "Balance",
            );
        });
        ui.separator();
        match &*account_actions {
            AccountActions::GetSeededAccount => {
                seeded::seeded_account_ui(ui, &mut account_input, &seeded_tx, &account_output)
            }
            AccountActions::GetAccountExists => {
                exists::account_exists_ui(ui, &mut account_input, &exists_tx, &account_output)
            }
            AccountActions::CreateAccount => {
                create::create_account_ui(ui, &mut account_input, &created_tx, &account_output)
            }
            AccountActions::FundAccount => {
                fund::account_fund_ui(ui, &mut account_input, &funded_tx, &account_output)
            }
            AccountActions::GetAccountBalance => {
                balance::account_balance_ui(ui, &mut account_input, &balance_tx, &account_output)   
            }
        }
    });
}

pub struct AccountPlugin;

impl Plugin for AccountPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AccountActions>()
            .init_resource::<AccountInputData>()
            .init_resource::<AccountOutputData>()
            .add_plugin(create::AccountCreatePlugin)
            .add_plugin(fund::AccountFundPlugin)
            .add_plugin(exists::AccountExistsPlugin)
            .add_plugin(seeded::SeededAccountPlugin)
            .add_plugin(balance::AccountBalancePlugin)
            .add_system(account_ui);
    }
}
