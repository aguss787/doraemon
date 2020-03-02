table! {
    url (key) {
        key -> Varchar,
        target -> Varchar,
    }
}

table! {
    user (id) {
        id -> Int4,
        username -> Varchar,
        password -> Varchar,
        salt -> Varchar,
    }
}

allow_tables_to_appear_in_same_query!(url, user,);
