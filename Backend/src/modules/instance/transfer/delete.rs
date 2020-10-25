use crate::modules::instance::dto::InstanceFailure;
use crate::modules::instance::Instance;
use crate::MainDb;
use rocket::State;
use crate::modules::account::guard::Authenticate;

#[openapi]
#[delete("/delete")]
pub fn delete_instance(mut db_main: MainDb, me: State<Instance>, auth: Authenticate) -> Result<(), InstanceFailure> {
    me.delete_instance(&mut *db_main, auth.0)
}