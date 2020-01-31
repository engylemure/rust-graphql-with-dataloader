table! {
    characters (id) {
        id -> Integer,
        name -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        deleted -> Bool,
    }
}

table! {
    movies (id) {
        id -> Integer,
        name -> Varchar,
        released_at -> Timestamp,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        deleted -> Bool,
    }
}

table! {
    movie_characters (id) {
        id -> Integer,
        movie_id -> Integer,
        character_id -> Integer,
    }
}

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

joinable!(movie_characters -> characters (character_id));
joinable!(movie_characters -> movies (movie_id));

allow_tables_to_appear_in_same_query!(
    characters,
    movies,
    movie_characters,
    users,
);
