// @generated automatically by Diesel CLI.

diesel::table! {
    remainders (user_id) {
        user_id -> Varchar,
        remainder_name -> Varchar,
        birthdate -> Date,
    }
}