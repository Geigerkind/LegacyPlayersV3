use crate::modules::armory::Armory;
use crate::modules::data::tools::RetrieveServer;
use crate::modules::data::Data as DataMaterial;
use crate::modules::live_data_processor::dto::LiveDataProcessorFailure;
use crate::modules::live_data_processor::material::{WoWRetailClassicParser, WoWTBCParser, WoWWOTLKParser, WoWVanillaParser};
use crate::modules::live_data_processor::tools::cbl_parser::CombatLogParser;
use crate::modules::live_data_processor::tools::log_parser::parse_cbl;
use crate::modules::live_data_processor::tools::ProcessMessages;
use crate::modules::live_data_processor::LiveDataProcessor;
use crate::util::database::{Execute, Select};
use crate::MainDb;
use chrono::NaiveDateTime;
use rocket::http::ContentType;
use rocket::{Data, State};
use rocket_multipart_form_data::{MultipartFormData, MultipartFormDataField, MultipartFormDataOptions, RawField};
use std::io::Read;
use crate::modules::account::guard::Authenticate;

#[openapi(skip)]
#[post("/upload", format = "multipart/form-data", data = "<form_data>")]
pub fn upload_log(mut db_main: MainDb, auth: Authenticate, me: State<LiveDataProcessor>, data: State<DataMaterial>, armory: State<Armory>, content_type: &ContentType, form_data: Data) -> Result<(), LiveDataProcessorFailure> {
    let mut options = MultipartFormDataOptions::new();
    options.allowed_fields.push(MultipartFormDataField::bytes("payload").size_limit(40 * 1024 * 1024 * 1024));
    options.allowed_fields.push(MultipartFormDataField::bytes("payload_armory").size_limit(10 * 1024 * 1024 * 1024));
    options.allowed_fields.push(MultipartFormDataField::bytes("server_id").size_limit(1024));
    options.allowed_fields.push(MultipartFormDataField::bytes("start_time").size_limit(1024));
    options.allowed_fields.push(MultipartFormDataField::bytes("end_time").size_limit(1024));

    let mut multipart_form_data = MultipartFormData::parse(content_type, form_data, options).unwrap();

    let mut start_time_raw_fields = multipart_form_data.raw.remove("start_time").ok_or(LiveDataProcessorFailure::InvalidInput)?;
    let start_time_raw_field = start_time_raw_fields.remove(0);
    let start_time_raw = std::str::from_utf8(&start_time_raw_field.raw).map_err(|_| LiveDataProcessorFailure::InvalidInput)?;
    let start_time = NaiveDateTime::parse_from_str(&start_time_raw, "%d.%m.%y %I:%M %p").ok().ok_or(LiveDataProcessorFailure::InvalidInput)?;

    let mut end_time_raw_fields = multipart_form_data.raw.remove("end_time").ok_or(LiveDataProcessorFailure::InvalidInput)?;
    let end_time_raw_field = end_time_raw_fields.remove(0);
    let end_time_raw = std::str::from_utf8(&end_time_raw_field.raw).map_err(|_| LiveDataProcessorFailure::InvalidInput)?;
    let end_time = NaiveDateTime::parse_from_str(&end_time_raw, "%d.%m.%y %I:%M %p").ok().ok_or(LiveDataProcessorFailure::InvalidInput)?;

    let mut server_id_raw_fields = multipart_form_data.raw.remove("server_id").ok_or(LiveDataProcessorFailure::InvalidInput)?;
    let RawField { raw: server_id_raw, .. } = server_id_raw_fields.remove(0);
    let server_id = i32::from_str_radix(std::str::from_utf8(&server_id_raw).map_err(|_| LiveDataProcessorFailure::InvalidInput)?, 10).map_err(|_| LiveDataProcessorFailure::InvalidInput)?;

    let mut raw_fields = multipart_form_data.raw.remove("payload").ok_or(LiveDataProcessorFailure::InvalidInput)?;
    let RawField { content_type: _, file_name: _, raw } = raw_fields.remove(0);
    if raw.is_empty() {
        return Err(LiveDataProcessorFailure::InvalidInput);
    }
    let reader = std::io::Cursor::new(raw.as_slice());
    let mut zip = zip::ZipArchive::new(reader).map_err(|_| LiveDataProcessorFailure::InvalidInput)?;

    // There should only be the combat log in there
    let file = zip.by_index(0).map_err(|_| LiveDataProcessorFailure::InvalidInput)?;
    let bytes = file.bytes().filter_map(|byte| byte.ok()).collect::<Vec<u8>>();
    let content = std::str::from_utf8(&bytes).map_err(|_| LiveDataProcessorFailure::InvalidInput)?;

    let armory_content = multipart_form_data.raw.remove("payload_armory").and_then(|mut armory_raw_fields| {
        let RawField { raw, .. } = armory_raw_fields.remove(0);
        std::str::from_utf8(&raw).ok().map(|str| str.to_string())
    });

    if server_id == -1 {
        return parse(&me, WoWRetailClassicParser::new(), &mut *db_main, &data, &armory, content, start_time.timestamp_millis() as u64, end_time.timestamp_millis() as u64, auth.0);
    } else {
        let server = data.get_server(server_id as u32).unwrap();
        if server.expansion_id == 1 {
            return parse(
                &me,
                WoWVanillaParser::new(server_id as u32),
                &mut *db_main,
                &data,
                &armory,
                content,
                start_time.timestamp_millis() as u64,
                end_time.timestamp_millis() as u64,
                auth.0
            );
        } else if server.expansion_id == 2 {
            return parse(
                &me,
                WoWTBCParser::new(server_id as u32, armory_content),
                &mut *db_main,
                &data,
                &armory,
                content,
                start_time.timestamp_millis() as u64,
                end_time.timestamp_millis() as u64,
                auth.0
            );
        } else if server.expansion_id == 3 {
            return parse(
                &me,
                WoWWOTLKParser::new(server_id as u32, armory_content),
                &mut *db_main,
                &data,
                &armory,
                content,
                start_time.timestamp_millis() as u64,
                end_time.timestamp_millis() as u64,
                auth.0
            );
        }
    }

    Err(LiveDataProcessorFailure::InvalidInput)
}

fn parse(me: &LiveDataProcessor, mut parser: impl CombatLogParser, db_main: &mut (impl Select + Execute), data: &DataMaterial, armory: &Armory, content: &str, start_time: u64, end_time: u64, member_id: u32) -> Result<(), LiveDataProcessorFailure> {
    if let Some((server_id, messages)) = parse_cbl(&mut parser, &mut *db_main, data, armory, content, start_time, end_time) {
        return me.process_messages(&mut *db_main, server_id as u32, &armory, &data, messages, member_id);
    }
    Err(LiveDataProcessorFailure::InvalidInput)
}
