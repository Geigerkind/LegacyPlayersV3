use rocket::{State, Data};
use rocket::http::ContentType;
use rocket_contrib::json::Json;
use crate::modules::live_data_processor::LiveDataProcessor;
use crate::modules::account::guard::ServerOwner;
use crate::modules::live_data_processor::dto::LiveDataProcessorFailure;
use crate::modules::live_data_processor::tools::ProcessMessages;

use rocket_multipart_form_data::{mime, MultipartFormDataOptions, MultipartFormData, MultipartFormDataField, Repetition, FileField, TextField, RawField};

#[openapi(skip)]
#[post("/package", format = "multipart/form-data", data = "<data>")]
pub fn get_package(me: State<LiveDataProcessor>, owner: ServerOwner, content_type: &ContentType, data: Data) -> Result<(), LiveDataProcessorFailure> {
  let mut options = MultipartFormDataOptions::new();
  options.allowed_fields.push(MultipartFormDataField::raw("payload").size_limit(40960));

  println!("{:?}", content_type);
  let multipart_form_data = MultipartFormData::parse(content_type, data, options).unwrap();

  let payload = multipart_form_data.raw.get("payload");

  println!("{:?}", payload);
  me.process_messages(owner.0, Vec::new())
}