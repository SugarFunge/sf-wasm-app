use bevy::prelude::*;
use bevy_egui::egui;
use crossbeam::channel;
use sugarfunge_api_types::{
    market::{CreateMarketInput, CreateMarketOutput},
    primitives::{MarketId, Seed},
};

use crate::{prelude::*, util::request_handler};

use super::MarketUi;

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

#[derive(Resource)]
pub struct CreateMarketChannel {
    pub input_tx: InputSender<CreateMarketRequest>,
    pub input_rx: InputReceiver<CreateMarketRequest>,
    pub output_tx: OutputSender<CreateMarketOutput>,
    pub output_rx: OutputReceiver<CreateMarketOutput>,
}

impl Default for CreateMarketChannel {
    fn default() -> Self {
        let (input_tx, input_rx) = channel::unbounded::<CreateMarketRequest>();
        let (output_tx, output_rx) = channel::unbounded::<Option<CreateMarketOutput>>();
        Self {
            input_tx: InputSender(input_tx),
            input_rx: InputReceiver(input_rx),
            output_tx: OutputSender(output_tx),
            output_rx: OutputReceiver(output_rx),
        }
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

pub fn create_market_ui(ui: &mut egui::Ui, market: &mut ResMut<MarketUi>) {
    ui.label("Create");
    ui.separator();
    ui.label("Seed");
    ui.text_edit_singleline(&mut *market.data.input.create_market.seed);
    ui.label("Market ID");
    ui.add(egui::DragValue::new(&mut *market.data.input.create_market.market_id).speed(1.0));
    ui.separator();
    if market.data.input.create_market.loading {
        ui.separator();
        ui.add(egui::Spinner::default());
    } else {
        if ui.button("Create").clicked() {
            market
                .channels
                .create_market
                .input_tx
                .0
                .send(CreateMarketRequest {
                    input: CreateMarketInput {
                        seed: market.data.input.create_market.seed.clone(),
                        market_id: market.data.input.create_market.market_id,
                    },
                })
                .unwrap();
            market.data.input.create_market.loading = true;
        }
    }
    if let Some(output) = &market.data.output.create_market {
        ui.separator();
        ui.label("Market ID");
        ui.text_edit_singleline(&mut u64::from(output.market_id).to_string());
        ui.label("Who");
        ui.text_edit_singleline(&mut output.who.as_str());
    }
}

pub fn handle_create_market_response(
    mut market: ResMut<MarketUi>,
    tokio_runtime: Res<TokioRuntime>,
) {
    if let Ok(created_result) = market.channels.create_market.output_rx.0.try_recv() {
        if let Some(created) = created_result {
            market.data.output.create_market = Some(created);
        }
        market.data.input.create_market.loading = false;
    }

    request_handler::<CreateMarketRequest, CreateMarketInput, CreateMarketOutput>(
        tokio_runtime.runtime.clone(),
        market.channels.create_market.input_rx.clone(),
        market.channels.create_market.output_tx.clone(),
    );
}

pub struct CreateMarketPlugin;

impl Plugin for CreateMarketPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(handle_create_market_response);
    }
}
