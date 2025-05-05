// @generated automatically by Diesel CLI.

diesel::table! {
    tokens (id) {
        id -> Int4,
        #[max_length = 255]
        token -> Varchar,
        created_at -> Nullable<Timestamp>,
        #[max_length = 45]
        ip_address -> Nullable<Varchar>,
    }
}

diesel::table! {
    utils (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 255]
        ai_apikey -> Varchar,
        created_at -> Nullable<Timestamp>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    tokens,
    utils,
);
