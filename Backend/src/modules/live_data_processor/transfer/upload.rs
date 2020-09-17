use crate::modules::armory::Armory;
use crate::modules::data::Data as DataMaterial;
use crate::modules::live_data_processor::dto::LiveDataProcessorFailure;
use crate::modules::live_data_processor::material::WoWCBTLParser;
use crate::modules::live_data_processor::tools::log_parser::LogParser;
use crate::modules::live_data_processor::tools::ProcessMessages;
use crate::modules::live_data_processor::LiveDataProcessor;
use crate::MainDb;
use rocket::{State, Data};
use rocket_multipart_form_data::{MultipartFormDataOptions, MultipartFormDataField, RawField, MultipartFormData};
use std::io::Read;
use rocket::http::ContentType;

#[openapi(skip)]
#[post("/upload", format = "multipart/form-data", data = "<form_data>")]
pub fn upload_log(mut db_main: MainDb, me: State<LiveDataProcessor>, data: State<DataMaterial>, armory: State<Armory>, content_type: &ContentType, form_data: Data) -> Result<(), LiveDataProcessorFailure> {
    let mut options = MultipartFormDataOptions::new();
    options.allowed_fields.push(MultipartFormDataField::bytes("payload").size_limit(20 * 1024 * 1024 * 1024));
    options.allowed_fields.push(MultipartFormDataField::bytes("server_id").size_limit(1024));

    let mut multipart_form_data = MultipartFormData::parse(content_type, form_data, options).unwrap();

    if let Some(mut server_id_raw_fields) = multipart_form_data.raw.remove("server_id") {
        let RawField { content_type: _, file_name: _, raw: server_id_raw } = server_id_raw_fields.remove(0);
        let server_id = u32::from_str_radix(std::str::from_utf8(&server_id_raw).map_err(|_| LiveDataProcessorFailure::InvalidInput)?, 10)
            .map_err(|_| LiveDataProcessorFailure::InvalidInput)?;
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

            let messages = WoWCBTLParser::new(&data, server_id, 0, u64::MAX, 0)
                .parse(&mut *db_main, &data, &armory, content);

            return me.process_messages(&mut *db_main, server_id, &armory, &data, messages);
        }
    }
    Err(LiveDataProcessorFailure::InvalidInput)
}
