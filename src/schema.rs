table! {
    events (id) {
        id -> Integer,
        name -> Text,
        date -> Text,
        location -> Text,
    }
}

table! {
    users (id) {
        id -> Integer,
        name -> Text,
        password -> Text,
    }
}
