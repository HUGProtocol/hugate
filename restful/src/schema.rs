// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        profile_image -> Nullable<Varchar>,
        email -> Nullable<Varchar>,
        twitter -> Nullable<Varchar>,
        about -> Nullable<Varchar>,
        pts -> Nullable<Int8>,
        create_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
    }
}
