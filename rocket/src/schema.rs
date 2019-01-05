table! {
    auctions (id) {
        id -> Integer,
        description -> Text,
        price -> Float,
        date -> Integer,
    }
}

table! {
    buynows (id) {
        id -> Integer,
        description -> Text,
        price -> Float,
        amount -> Integer,
    }
}

table! {
    users (mail) {
        mail -> Text,
        password -> Text,
    }
}

allow_tables_to_appear_in_same_query!(
    auctions,
    buynows,
    users,
);
