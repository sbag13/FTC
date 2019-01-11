table! {
    offers (id) {
        id -> Integer,
        owner -> Text,
        #[sql_name = "type"]
        type_ -> Text,
        description -> Text,
        price -> Float,
        date_amount -> Integer,
    }
}

table! {
    transactions (id) {
        id -> Integer,
        offer_id -> Integer,
        buyer -> Text,
        amount -> Nullable<Integer>,
        bid -> Nullable<Float>,
    }
}

table! {
    users (mail) {
        mail -> Text,
        password -> Text,
    }
}

joinable!(offers -> users (owner));
joinable!(transactions -> offers (offer_id));
joinable!(transactions -> users (buyer));

allow_tables_to_appear_in_same_query!(
    offers,
    transactions,
    users,
);
