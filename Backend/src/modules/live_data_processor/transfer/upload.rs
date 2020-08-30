use crate::modules::armory::Armory;
use crate::modules::data::Data;
use crate::modules::live_data_processor::dto::LiveDataProcessorFailure;
use crate::modules::live_data_processor::material::WoWCBTLParser;
use crate::modules::live_data_processor::tools::log_parser::LogParser;
use crate::modules::live_data_processor::tools::ProcessMessages;
use crate::modules::live_data_processor::LiveDataProcessor;
use crate::MainDb;
use rocket::State;

#[openapi]
#[get("/upload")]
pub fn upload_log(mut db_main: MainDb, me: State<LiveDataProcessor>, data: State<Data>, armory: State<Armory>) -> Result<(), LiveDataProcessorFailure> {
    let messages = WoWCBTLParser::new(&data, 6, 0, u64::MAX, 0).parse(&mut *db_main, &data, &armory, "/home/rpll/+0100_Hayleigh_000_100_Feierabend_2020_08_27_naxx_25.txt");

    me.process_messages(&mut *db_main, 6, &armory, &data, messages)
}
