table! {
    client_credential (id) {
        id -> Varchar,
        secret -> Varchar,
        redirect_uri -> Varchar,
    }
}

table! {
    url (key) {
        key -> Varchar,
        target -> Varchar,
        username -> Varchar,
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

allow_tables_to_appear_in_same_query!(client_credential, url, user,);
