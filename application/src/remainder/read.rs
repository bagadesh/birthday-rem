use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};

use domain::models::Remainder;
use domain::schema::remainders;
//use domain::schema::remainders::dsl::remainders;
use infra::establish_connection;

pub fn get_remainders(user_id: String) -> Vec<Remainder> {
    let connection = &mut establish_connection();
    let a = remainders::table
        .filter(remainders::user_id.eq(user_id))
        .load::<Remainder>(connection);

    // let a = remainders.load(connection);

    match a {
        Ok(rs) => {
            rs
        }
        Err(err) => {
            match err {
                _ => {
                    panic!("Failed to get remainders")
                }
            }
        }
    }
}