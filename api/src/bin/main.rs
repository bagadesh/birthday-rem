use rocket::{launch, routes};
use api::remainder_handler;

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/api", routes![
        remainder_handler::get_remainders,
        remainder_handler::add_remainder,
        remainder_handler::update_remainder,
        remainder_handler::delete_remainder,
    ])
}