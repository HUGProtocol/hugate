// @generated automatically by Diesel CLI.

diesel::table! {
    comment (id) {
        id -> Int4,
        thought_id -> Int4,
        address -> Varchar,
        content -> Varchar,
        create_at -> Timestamptz,
    }
}

diesel::table! {
    follow (id) {
        id -> Int4,
        follower -> Varchar,
        followee -> Varchar,
        create_at -> Timestamptz,
    }
}

diesel::table! {
    likes (id) {
        id -> Int4,
        thought_id -> Int4,
        address -> Varchar,
        create_at -> Timestamptz,
    }
}

diesel::table! {
    thoughts (id) {
        id -> Int4,
        content -> Varchar,
        address -> Varchar,
        tips -> Varchar,
        thought_type -> Varchar,
        source_url -> Varchar,
        snapshot -> Varchar,
        create_at -> Timestamptz,
        updated_at -> Timestamptz,
        likes -> Int8,
        viewed -> Varchar,
        submit_state -> Varchar,
        html -> Varchar,
        pts -> Int8,
        embeded -> Varchar,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        profile_image -> Varchar,
        email -> Varchar,
        twitter -> Varchar,
        about -> Varchar,
        pts -> Int8,
        create_at -> Timestamptz,
        updated_at -> Timestamptz,
        address -> Varchar,
        banner -> Varchar,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    comment,
    follow,
    likes,
    thoughts,
    users,
);
