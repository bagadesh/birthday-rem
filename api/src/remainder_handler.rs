use rocket::{delete, get, post, put};
use rocket::response::status::{Accepted, Created};
use rocket::serde::json::Json;
use application::remainder::{delete, read, update};
use application::remainder::create;
use domain::models::{ModifyRemainder, Remainder};
use shared::response_models::{Response, ResponseBody};


#[get("/remainders/<user_id>")]
pub fn get_remainders(user_id: String) -> String {
    let remainders = read::get_remainders(user_id);
    let response = Response { body: ResponseBody::Remainders(remainders) };

    serde_json::to_string(&response).unwrap()
}

#[post("/add_remainder", format = "application/json", data = "<remainder>")]
pub fn add_remainder(remainder: Json<Remainder>) -> Created<String> {
    let remainder = create::add_remainder(remainder);
    let response = Response { body: ResponseBody::Remainder(remainder) };
    Created::new("").tagged_body(serde_json::to_string(&response).unwrap())
}

#[put("/modify_remainder", format = "application/json", data = "<remainder>")]
pub fn update_remainder(remainder: Json<ModifyRemainder>) -> Accepted<String> {
    let remainder = update::update_remainder(remainder);
    let response = Response { body: ResponseBody::Remainder(remainder) };
    Accepted(Some(serde_json::to_string(&response).unwrap()))
}

#[delete("/<user_id>/<remainder_name>")]
pub fn delete_remainder(user_id: String, remainder_name: String) -> Accepted<String> {
    let result = delete::delete_remainder(user_id, remainder_name);
    let response = Response { body: ResponseBody::Message(result) };
    Accepted(Some(serde_json::to_string(&response).unwrap()))
}