// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Integer,
        fname -> Varchar,
        lname -> Nullable<Varchar>,
        email -> Varchar,
        is_admin -> Bool,
        created_at -> Timestamp,
    }
}
