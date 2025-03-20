// @generated automatically by Diesel CLI.

use diesel::table;

diesel::table! {
    deposits (id) {
        id -> Nullable<Integer>,
        token -> Text,
        sender -> Text,
        recipient -> Text,
        amount -> Text,
        nonce -> Integer,
        processed -> Nullable<Bool>,
    }
}

table! {
    distributions (id) {
        id -> Integer,
        token -> Text,
        sender -> Text,
        recipient -> Text,
        amount -> Text,
        nonce -> Integer,
        processed -> Bool,
    }
}


diesel::allow_tables_to_appear_in_same_query!(
    deposits,
    distributions,
);
