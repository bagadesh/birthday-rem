use rocket::serde::Serialize;
use domain::models::{Remainder};

#[derive(Serialize)]
pub enum ResponseBody {
    Message(String),
    Remainder(Remainder),
    Remainders(Vec<Remainder>),
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Response {
    pub body: ResponseBody,
}