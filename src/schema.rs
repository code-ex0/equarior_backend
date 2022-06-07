table! {
    games (id) {
        id -> Int4,
        ids_players -> Array<Int4>,
        created_at -> Timestamptz,
    }
}

table! {
    rounds (id) {
        id -> Int4,
        game_id -> Int4,
        data -> Text,
        created_at -> Timestamptz,
    }
}

table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        email -> Varchar,
        password -> Varchar,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

joinable!(rounds -> games (game_id));

allow_tables_to_appear_in_same_query!(
    games,
    rounds,
    users,
);
