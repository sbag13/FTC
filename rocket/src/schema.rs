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
    users (mail) {
        mail -> Text,
        password -> Text,
    }
}

joinable!(offers -> users (owner));

allow_tables_to_appear_in_same_query!(
    offers,
    users,
);
