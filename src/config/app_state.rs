use mongodb::{Client, Database};
use crate::machine_learning::network::Network;

#[derive(Debug, Clone)]
pub struct AppState {
    pub db: Database,
    pub network: f64,
    // pub network: Network<'static>,
}
