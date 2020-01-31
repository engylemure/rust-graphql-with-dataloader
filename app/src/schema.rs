table! {
    users (id) {
        id -> Integer,
        hash -> Blob,
        uuid -> Varchar,
        salt -> Varchar,
        email -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        deleted -> Bool,
    }
}
