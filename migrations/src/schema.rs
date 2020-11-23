table! {
    events (id) {
        id -> Int4,
        name -> Text,
        date -> Text,
        location -> Text,
    }
}

table! {
    sessions (id) {
        id -> Int4,
        sessionid -> Text,
    }
}

table! {
    users (id) {
        id -> Int4,
        name -> Text,
        password -> Text,
    }
}

allow_tables_to_appear_in_same_query!(
    events,
    sessions,
    users,
);
