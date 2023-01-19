use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};
use sugarfunge_api_types::account::*;

use crate::prelude::*;

pub mod create;
pub mod fund;

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
    fund_account: fund::FundAccountInputData,
    // account_exists: Option<AccountExistsInput>,
    // account_balance: Option<AccountBalanceInput>,
    // seeded_account: Option<SeededAccountInput>,
}

#[derive(Resource, Default, Debug)]
pub struct AccountOutputData {
    create_account: Option<CreateAccountOutput>,
    fund_account: Option<FundAccountOutput>,
}

pub fn account_ui(
    mut ctx: ResMut<EguiContext>,
    mut account_actions: ResMut<AccountActions>,
    mut account_input: ResMut<AccountInputData>,
    account_output: Res<AccountOutputData>,
    created_tx: Res<InputSender<create::CreateAccountRequest>>,
    funded_tx: Res<InputSender<fund::FundAccountRequest>>,
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
            AccountActions::GetSeededAccount => {}
            AccountActions::GetAccountExists => {}
            AccountActions::CreateAccount => {
                create::create_account_ui(ui, &created_tx, &account_output)
            }
            AccountActions::FundAccount => {
                fund::account_fund_ui(ui, &mut account_input, &funded_tx, &account_output)
            }
            AccountActions::GetAccountBalance => {}
        }
    });
}

fn handle_account_response(
    mut account: ResMut<AccountOutputData>,
    created_rx: Res<OutputReceiver<CreateAccountOutput>>,
    funded_rx: Res<OutputReceiver<FundAccountOutput>>,
) {
    if let Ok(created) = created_rx.0.try_recv() {
        account.create_account = Some(created);
    }
    if let Ok(funded) = funded_rx.0.try_recv() {
        account.fund_account = Some(funded);
    }
}

pub struct AccountPlugin;

impl Plugin for AccountPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AccountActions>()
            .init_resource::<AccountInputData>()
            .init_resource::<AccountOutputData>()
            .add_plugin(create::AccountCreatePlugin)
            .add_plugin(fund::AccountFundPlugin)
            .add_system(account_ui)
            .add_system(handle_account_response);
    }
}
