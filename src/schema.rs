// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Int4,
        name -> Varchar,
        avatar -> Varchar,
        created_at -> Date,
    }
}
