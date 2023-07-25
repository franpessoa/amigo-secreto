// @generated automatically by Diesel CLI.

diesel::table! {
    jogadores (id) {
        id -> Integer,
        name -> Text,
        email -> Text,
        game_id -> Integer,
    }
}

diesel::table! {
    jogos (id) {
        id -> Integer,
        name -> Text,
        created_at -> Timestamp,
        hashed_password -> Text,
        game_password -> Nullable<Text>,
    }
}

diesel::joinable!(jogadores -> jogos (game_id));

diesel::allow_tables_to_appear_in_same_query!(
    jogadores,
    jogos,
);
