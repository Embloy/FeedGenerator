use mongodb::{Database};
// use crate::machine_learning::network::Network;

#[derive(Debug, Clone)]
pub struct AppState {
    pub db: Database,
    pub network: String,
    // pub network: Network<'static>,
}
