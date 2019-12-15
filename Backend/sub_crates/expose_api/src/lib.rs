extern crate serde;
extern crate serde_json;
extern crate str_util;

use serde::ser::Serialize;
use str_util::strformat;

pub fn expose_api_fn<T: Serialize, U: Serialize>(url: &str, request_type: &str, requires_authentication: bool, response_type: &str, schema_response: T, schema_arg1: U) -> serde_json::Value
{
  let template: String = r#"{"url":"{0}","request_type":"{1}","":{2},"args":[{3}],"response_type":"{4}","response":{5}}"#.to_string();
  serde_json::from_str(&strformat::fmt(template, &[url, request_type, &serde_json::to_string_pretty(&requires_authentication).unwrap(), &serde_json::to_string_pretty(&schema_arg1).unwrap(), response_type, &serde_json::to_string_pretty(&schema_response).unwrap()])).unwrap()
}