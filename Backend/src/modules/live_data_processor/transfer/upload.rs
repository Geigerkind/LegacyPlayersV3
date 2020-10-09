use crate::modules::armory::Armory;
use crate::modules::data::Data as DataMaterial;
use crate::modules::live_data_processor::dto::LiveDataProcessorFailure;
use crate::modules::live_data_processor::material::WoWCBTLParser;
use crate::modules::live_data_processor::tools::log_parser::LogParser;
use crate::modules::live_data_processor::tools::ProcessMessages;
use crate::modules::live_data_processor::LiveDataProcessor;
use crate::MainDb;
use chrono::NaiveDateTime;
use rocket::http::ContentType;
use rocket::{Data, State};
use rocket_multipart_form_data::{MultipartFormData, MultipartFormDataField, MultipartFormDataOptions, RawField};
use std::io::Read;

#[openapi(skip)]
#[post("/upload", format = "multipart/form-data", data = "<form_data>")]
pub fn upload_log(mut db_main: MainDb, me: State<LiveDataProcessor>, data: State<DataMaterial>, armory: State<Armory>, content_type: &ContentType, form_data: Data) -> Result<(), LiveDataProcessorFailure> {
    let mut options = MultipartFormDataOptions::new();
    options.allowed_fields.push(MultipartFormDataField::bytes("payload").size_limit(20 * 1024 * 1024 * 1024));
    options.allowed_fields.push(MultipartFormDataField::bytes("server_id").size_limit(1024));
    options.allowed_fields.push(MultipartFormDataField::bytes("start_time").size_limit(1024));
    options.allowed_fields.push(MultipartFormDataField::bytes("end_time").size_limit(1024));

    let mut multipart_form_data = MultipartFormData::parse(content_type, form_data, options).unwrap();

    if let Some(mut start_time_raw_fields) = multipart_form_data.raw.remove("start_time") {
        let start_time_raw_field = start_time_raw_fields.remove(0);
        let start_time_raw = std::str::from_utf8(&start_time_raw_field.raw).map_err(|_| LiveDataProcessorFailure::InvalidInput)?;
        let start_time = NaiveDateTime::parse_from_str(&start_time_raw, "%d.%m.%y %I:%M %p").ok().ok_or(LiveDataProcessorFailure::InvalidInput)?;
        if let Some(mut end_time_raw_fields) = multipart_form_data.raw.remove("end_time") {
            let end_time_raw_field = end_time_raw_fields.remove(0);
            let end_time_raw = std::str::from_utf8(&end_time_raw_field.raw).map_err(|_| LiveDataProcessorFailure::InvalidInput)?;
            let end_time = NaiveDateTime::parse_from_str(&end_time_raw, "%d.%m.%y %I:%M %p").ok().ok_or(LiveDataProcessorFailure::InvalidInput)?;
            if let Some(mut server_id_raw_fields) = multipart_form_data.raw.remove("server_id") {
                let RawField {
                    content_type: _,
                    file_name: _,
                    raw: server_id_raw,
                } = server_id_raw_fields.remove(0);
                let server_id = i32::from_str_radix(std::str::from_utf8(&server_id_raw).map_err(|_| LiveDataProcessorFailure::InvalidInput)?, 10).map_err(|_| LiveDataProcessorFailure::InvalidInput)?;
                if let Some(mut raw_fields) = multipart_form_data.raw.remove("payload") {
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

                    let mut parser = WoWCBTLParser::new(&data, server_id, start_time.timestamp_millis() as u64, end_time.timestamp_millis() as u64, 0);
                    let messages = parser.parse(&mut *db_main, &data, &armory, content);

                    if parser.server_id < 0 {
                        return Err(LiveDataProcessorFailure::InvalidInput);
                    }

                    return me.process_messages(&mut *db_main, parser.server_id as u32, &armory, &data, messages);
                }
            }
        }
    }
    Err(LiveDataProcessorFailure::InvalidInput)
}
