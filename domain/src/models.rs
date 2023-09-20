use chrono::NaiveDate;
use diesel::{AsChangeset, Identifiable, Insertable, Queryable};
use rocket::serde::{Deserialize, Serialize};

use crate::schema::remainders;

#[derive(Queryable, Serialize, Ord, Eq, PartialEq, PartialOrd, Deserialize, Insertable)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = remainders)]
pub struct Remainder {
    pub user_id: String,
    pub remainder_name: String,
    pub birthdate: NaiveDate,
}

#[derive(Deserialize, Identifiable, AsChangeset, Serialize)]
#[diesel(table_name = remainders)]
#[primary_key(user_id)]
pub struct ModifyRemainder {
    pub user_id: String,
    pub remainder_name: Option<String>,
    pub birthdate: Option<NaiveDate>,
}

// #[derive(Insertable, Serialize, Deserialize, Ord, Eq, PartialEq, PartialOrd)]
// #[serde(crate = "rocket::serde")]
// #[diesel(table_name = remainders)]
// pub struct NewRemainder {
//     pub user_id: String,
//     pub remainder_name: String,
//     pub birthdate: NaiveDate,
// }