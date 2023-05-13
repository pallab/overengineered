// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Integer,
        fname -> Varchar,
        lname -> Nullable<Varchar>,
        email -> Nullable<Varchar>,
        is_admin -> Bool,
        created_at -> Nullable<Timestamp>,
    }
}
