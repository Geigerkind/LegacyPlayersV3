use std::io::Read;

use rocket::{Request, Data, Outcome, Outcome::*};
use rocket::data::FromDataSimple;
use rocket::http::{Status, ContentType};

pub struct RawString {
    pub content: String
}

const LIMIT: u64 = 1024;
impl FromDataSimple for RawString {
    type Error = String;

    fn from_data(req: &Request, data: Data) -> rocket::data::Outcome<Self, String> {
        let person_ct = ContentType::new("application", "json");
        if req.content_type() != Some(&person_ct) {
            return Outcome::Forward(data);
        }

        // Read the data into a String.
        let mut string = String::new();
        if let Err(e) = data.open().take(LIMIT).read_to_string(&mut string) {
            return Failure((Status::InternalServerError, format!("{:?}", e)));
        }

        Success(RawString {
            content: string
        })
    }
}