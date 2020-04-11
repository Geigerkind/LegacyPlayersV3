use rocket::{State, Data};
use rocket::http::ContentType;
use crate::modules::live_data_processor::LiveDataProcessor;
use crate::modules::account::guard::ServerOwner;
use crate::modules::live_data_processor::dto::LiveDataProcessorFailure;
use crate::modules::live_data_processor::tools::ProcessMessages;

use rocket_multipart_form_data::{MultipartFormDataOptions, MultipartFormData, MultipartFormDataField, SingleRawField};
use rocket_multipart_form_data::RawField::Single;
use crate::modules::armory::Armory;

#[openapi(skip)]
#[post("/package", format = "multipart/form-data", data = "<data>")]
pub fn get_package(me: State<LiveDataProcessor>, armory: State<Armory>, owner: ServerOwner, content_type: &ContentType, data: Data) -> Result<(), LiveDataProcessorFailure> {
  let mut options = MultipartFormDataOptions::new();
  options.allowed_fields.push(MultipartFormDataField::bytes("payload").size_limit(2 * 1024 * 1024));

  let mut multipart_form_data = MultipartFormData::parse(content_type, data, options).unwrap();

  let payload = multipart_form_data.raw.get_mut("payload");

  if let Some(Single(SingleRawField { content_type: _, file_name: _, raw })) = payload {
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
    return me.process_messages(owner.0, &armory, messages);
  }
  Err(LiveDataProcessorFailure::InvalidInput)
}