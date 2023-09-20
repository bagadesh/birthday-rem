use diesel::RunQueryDsl;
use rocket::serde::json::Json;

use domain::models::{ModifyRemainder, Remainder};
use infra::establish_connection;

pub fn update_remainder(remainder: Json<ModifyRemainder>) -> Remainder {
    let remainder = remainder.into_inner();
    let connection = &mut establish_connection();
    let result = diesel::update(&remainder).set(&remainder).get_result::<Remainder>(connection);
    match result {
        Ok(rm) => {
            rm
        }
        Err(err) => {
            match err {
                _ => {
                    panic!("Failed to update")
                }
            }
        }
    }
}