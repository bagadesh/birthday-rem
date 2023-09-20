// @generated automatically by Diesel CLI.

diesel::table! {
    remainders (user_id) {
        user_id -> Int4,
        remainder_name -> Varchar,
        birthdate -> Date,
    }
}