// @generated automatically by Diesel CLI.

diesel::table! {
    todos (id) {
        id -> Integer,
        #[max_length = 255]
        title -> Varchar,
        description -> Text,
        created -> Datetime,
        updated -> Datetime,
    }
}
