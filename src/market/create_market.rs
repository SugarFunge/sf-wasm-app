use bevy::prelude::*;
use bevy_egui::egui;
use sugarfunge_api_types::{
    market::{CreateMarketInput, CreateMarketOutput},
    primitives::{MarketId, Seed},
};

use crate::{prelude::*, util::*};

use super::{MarketInputData, MarketOutputData};

#[derive(Debug)]
pub struct CreateMarketRequest {
    pub input: CreateMarketInput,
}

impl Request<CreateMarketInput> for CreateMarketRequest {
    fn endpoint(&self) -> &str {
        "market/create_market"
    }

    fn input(&self) -> Option<CreateMarketInput> {
        Some(CreateMarketInput {
            seed: self.input.seed.clone(),
            market_id: self.input.market_id.clone(),
        })
    }
}

#[derive(Resource, Debug, Clone)]
pub struct CreateMarketInputData {
    pub seed: Seed,
    pub market_id: MarketId,
    pub loading: bool,
}

impl Default for CreateMarketInputData {
    fn default() -> Self {
        Self {
            seed: Seed::from("".to_string()),
            market_id: MarketId::from(0),
            loading: false,
        }
    }
}

pub fn create_market_ui(
    ui: &mut egui::Ui,
    market_input: &mut ResMut<MarketInputData>,
    create_tx: &Res<InputSender<CreateMarketRequest>>,
    market_output: &Res<MarketOutputData>,
) {
    ui.label("Seed");
    ui.text_edit_singleline(&mut *market_input.create_market_input.seed);
    ui.label("Market ID");
    ui.add(egui::DragValue::new(&mut *market_input.create_market_input.market_id).speed(1.0));
    ui.separator();
    if ui.button("Create").clicked() {
        create_tx
            .send(CreateMarketRequest {
                input: CreateMarketInput {
                    seed: market_input.create_market_input.seed.clone(),
                    market_id: market_input.create_market_input.market_id,
                },
            })
            .unwrap();
        market_input.create_market_input.loading = true;
    }
    if let Some(output) = &market_output.create_market_output {
        ui.separator();
        ui.label("Market ID");
        ui.text_edit_singleline(&mut u64::from(output.market_id).to_string());
        ui.label("Who");
        ui.text_edit_singleline(&mut output.who.as_str());
    }
}

pub fn handle_create_market_response(
    mut market_output: ResMut<MarketOutputData>,
    mut market_input: ResMut<MarketInputData>,
    created_rx: Res<OutputReceiver<CreateMarketOutput>>,
) {
    if let Ok(created_result) = created_rx.0.try_recv() {
        if let Some(created) = created_result {
            market_output.create_market_output = Some(created);
        }
        market_input.create_market_input.loading = false;
    }
}

pub struct CreateMarketPlugin;

impl Plugin for CreateMarketPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_in_out_channels::<CreateMarketRequest, CreateMarketOutput>)
            .add_system(
                request_handler::<CreateMarketRequest, CreateMarketInput, CreateMarketOutput>,
            )
            .add_system(handle_create_market_response);
    }
}
