use crate::modules::live_data_processor::material::Server;
use crate::util::database::{Execute, Select};

impl Server {
    pub fn perform_post_processing(&mut self, db_main: &mut (impl Execute + Select)) {
        // TODO: Save committed events to disk
        // TODO: Set end_ts of instance
        // TODO: Extract Attempts?
        // TODO: Extract Ranking
    }
}
