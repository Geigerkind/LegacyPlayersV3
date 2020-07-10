use crate::modules::account::guard::ServerOwner;
use crate::modules::live_data_processor::dto::LiveDataProcessorFailure;
use crate::modules::live_data_processor::tools::ProcessMessages;
use crate::modules::live_data_processor::LiveDataProcessor;
use rocket::http::ContentType;
use rocket::{Data, State};

use crate::modules::armory::Armory;
use rocket_multipart_form_data::{MultipartFormData, MultipartFormDataField, MultipartFormDataOptions, RawField};
use crate::MainDb;

#[openapi(skip)]
#[post("/package", format = "multipart/form-data", data = "<data>")]
pub fn get_package(mut db_main: MainDb, me: State<LiveDataProcessor>, armory: State<Armory>, owner: ServerOwner, content_type: &ContentType, data: Data) -> Result<(), LiveDataProcessorFailure> {
    let mut options = MultipartFormDataOptions::new();
    options.allowed_fields.push(MultipartFormDataField::bytes("payload").size_limit(2 * 1024 * 1024));

    let mut multipart_form_data = MultipartFormData::parse(content_type, data, options).unwrap();

    let payload = multipart_form_data.raw.get_mut("payload");

    if let Some(raw_fields) = payload {
        // TODO: Test this new behavior
        if let Some(raw_field) = raw_fields.get_mut(0) {
            let RawField { content_type: _, file_name: _, raw } = raw_field;
            if raw.is_empty() {
                println!("Is empty!");
                return Err(LiveDataProcessorFailure::InvalidInput);
            }

            let mut messages = Vec::new();
            while !raw.is_empty() {
                if raw[2] == 0 {
                    return Err(LiveDataProcessorFailure::InvalidInput);
                }
                messages.push(raw.drain(..(raw[2] as usize)).collect());
            }
            println!("Messages: {:?}", messages);
            return me.process_messages(&mut *db_main, owner.0, &armory, messages);
        }
    }
    Err(LiveDataProcessorFailure::InvalidInput)
}
