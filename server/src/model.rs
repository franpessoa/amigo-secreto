use diesel::prelude::*;
use chrono::NaiveDateTime;
use crate::schema::{
    jogos, jogadores
};

#[derive(Queryable, Identifiable, Selectable, Debug, PartialEq)]
#[diesel(table_name=jogos)]
pub struct Jogo {
    pub id: i32, 
    pub name: String,
    pub created_at: NaiveDateTime,
    pub hashed_password: String,
    pub game_password: Option<String>
}

#[derive(Queryable, Selectable, Identifiable, Associations, Debug, PartialEq)]
#[diesel(belongs_to(Jogo, foreign_key=game_id))]
#[diesel(table_name = jogadores)]
pub struct Jogadores {
    pub id: i32,
    pub name: String, 
    pub email: String,
    pub game_id: i32
}
