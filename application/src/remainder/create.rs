use diesel::{Insertable, RunQueryDsl};
use rocket::serde::json::Json;

use domain::models::{Remainder};
use domain::schema::remainders;
use infra::establish_connection;

pub fn add_remainder(remainder: Json<Remainder>) -> Remainder {
    let remainder = remainder.into_inner();
    let connection = &mut establish_connection();
    let query = diesel::insert_into(remainders::table).values(remainder).get_result::<Remainder>(connection);
    match query {
        Ok(res) => {
            res
        }
        Err(err) => {
            match err {
                _ => panic!("Failed to add remainder")
            }
        }
    }
}