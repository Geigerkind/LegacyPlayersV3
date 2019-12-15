extern crate expose_api;

use expose_api::expose_api_fn;
use rocket_contrib::json::Json;
use schemars::{JsonSchema, schema_for};

use crate::modules::account::domain_value::AccountInformation;
use crate::modules::account::dto::{CreateMember, CreateToken, Credentials, Failure, ProlongToken};
use crate::modules::account::material::APIToken;

#[derive(JsonSchema)]
struct Nothing;

#[get("/", format = "application/json")]
pub fn api() -> Json<Vec<serde_json::Value>> {
  Json(vec![
    // Get
    expose_api_fn("/get/", "get", true, "application/json", schema_for!(Result<AccountInformation, Failure>), schema_for!(Nothing)),

    // Create
    expose_api_fn("/create/<create_member>", "post", false, "application/json", schema_for!(Result<APIToken, Failure>), schema_for!(CreateMember)),
    expose_api_fn("/create/<confirmation_id>", "get", false, "application/json", schema_for!(Result<(), Failure>), schema_for!(String)),
    expose_api_fn("/create/resend", "get", true, "application/json", schema_for!(Result<(), Failure>), schema_for!(Nothing)),

    // Delete
    expose_api_fn("/delete/<delete_id>", "get", false, "text/plain", schema_for!(Result<(), Failure>), schema_for!(String)),
    expose_api_fn("/delete", "delete", true, "application/json", schema_for!(Result<(), Failure>), schema_for!(Nothing)),

    // Forgot
    expose_api_fn("/forgot/<confirmation_id>", "get", false, "text/plain", schema_for!(Result<APIToken, Failure>), schema_for!(String)),
    expose_api_fn("/forgot/<mail>", "post", false, "application/json", schema_for!(Result<(), Failure>), schema_for!(String)),

    // Login
    expose_api_fn("/login/<credentials>", "post", false, "application/json", schema_for!(Result<APIToken, Failure>), schema_for!(Credentials)),

    // Update
    expose_api_fn("/update/password/<password>", "post", true, "application/json", schema_for!(Result<APIToken, Failure>), schema_for!(String)),
    expose_api_fn("/update/nickname/<nickname>", "post", true, "application/json", schema_for!(Result<AccountInformation, Failure>), schema_for!(String)),
    expose_api_fn("/update/mail/<mail>", "post", true, "application/json", schema_for!(Result<bool, Failure>), schema_for!(String)),
    expose_api_fn("/update/mail/<confirmation_id>", "get", false, "application/json", schema_for!(Result<APIToken, Failure>), schema_for!(String)),

    // Token
    expose_api_fn("/token/<create_token>", "post", true, "application/json", schema_for!(Result<APIToken, Failure>), schema_for!(CreateToken)),
    expose_api_fn("/token", "get", true, "application/json", schema_for!(Result<Vec<APIToken>, Failure>), schema_for!(Nothing)),
    expose_api_fn("/token/<token_id>", "post", true, "application/json", schema_for!(Result<(), Failure>), schema_for!(u32)),
    expose_api_fn("/token/prolong/<prolong_token>", "post", true, "application/json", schema_for!(Result<APIToken, Failure>), schema_for!(ProlongToken)),
  ])
}