// @generated automatically by Diesel CLI.

diesel::table! {
    account (id) {
        id -> Integer,
        #[max_length = 255]
        username -> Varchar,
        #[max_length = 255]
        password -> Varchar,
        #[max_length = 255]
        email -> Nullable<Varchar>,
        created_at -> Nullable<Datetime>,
        updated_at -> Nullable<Datetime>,
    }
}
