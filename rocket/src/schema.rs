table! {
    offers (id) {
        id -> Integer,
        #[sql_name = "type"]
        type_ -> Text,
        description -> Text,
        price -> Float,
        date_amount -> Integer,
    }
}

table! {
    owners (mail, id) {
        mail -> Text,
        id -> Integer,
    }
}

table! {
    users (mail) {
        mail -> Text,
        password -> Text,
    }
}

allow_tables_to_appear_in_same_query!(
    offers,
    owners,
    users,
);
