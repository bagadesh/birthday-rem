use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};

use domain::schema::remainders::dsl::remainders;
use domain::schema::remainders::remainder_name;
use domain::schema::remainders::user_id;
use infra::establish_connection;

pub fn delete_remainder(id: String, name: String) -> String {
    let connection = &mut establish_connection();
    let result = diesel::delete(remainders
        .filter(user_id.eq(id))
        .filter(remainder_name.eq(name))).execute(connection);
    match result {
        Ok(count) => {
            format!("Successfully able to delete {} records", count)
        }
        Err(err) => {
            match err {
                _ => {
                    panic!("Cannot able to delete given record")
                }
            }
        }
    }
}