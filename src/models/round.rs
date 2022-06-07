use rocket::serde::{Deserialize, Serialize};
use diesel::{
    prelude::*,
    AsChangeset,
    Insertable,
    Queryable
};
use rocket::serde::json::Json ;
use crate::{
    schema::rounds,
    PgConnection
};
use crate::schema::users;

///
/// Round model
/// used to store the rounds of a game
///
#[derive(Debug, Serialize, Queryable)]
#[serde(crate = "rocket::serde")]
pub struct Round {
    pub id: i32,
    pub game_id: i32,
    pub data: String,
    pub created_at: chrono::NaiveDateTime,
}

///
/// Round model
/// used to create new rounds
///
#[derive(Debug, Deserialize, Insertable)]
#[serde(crate = "rocket::serde")]
#[table_name="rounds"]
pub struct NewRound {
    pub game_id: i32,
    pub data: String,
}

///
/// Round model
/// used to update rounds
///
#[derive(Debug, Deserialize, AsChangeset)]
#[serde(crate = "rocket::serde")]
#[table_name="rounds"]
pub struct UpdateRound {
    pub data: String,
}

impl Round {
    ///
    /// insert new round
    /// - `new_game`: [NewRound]
    /// - `conn`: [PgConnection]
    /// - Returns: [Round, diesel::result::Error]
    ///
    pub async fn insert(conn: &PgConnection, new_round: Json<NewRound>) -> Result<Round, diesel::result::Error> {
        Ok(conn.run(move |c| diesel::insert_into(rounds::table).values(new_round.into_inner()).get_result(c)).await?)
    }

    ///
    /// get all rounds
    /// - `conn`: [PgConnection]
    /// - Returns: [Vec<Round>, diesel::result::Error]
    ///
    pub async fn get_all(conn: &PgConnection) -> Result<Vec<Round>, diesel::result::Error> {
        Ok(conn.run(move |c| rounds::table.load(c)).await?)
    }

    ///
    /// get round by id
    /// - `conn`: [PgConnection]
    /// - `id`: [i32]
    /// - Returns: [Round, diesel::result::Error]
    ///
    pub async fn get_by_id(conn: &PgConnection, id: i32) -> Result<Round, diesel::result::Error> {
        Ok(conn.run(move |c| rounds::table.filter(rounds::id.eq(id)).get_result(c)).await?)
    }

    ///
    /// get round by game id
    /// - `conn`: [PgConnection]
    /// - `id`: [i32]
    /// - Returns: [Vec<Round>, diesel::result::Error]
    ///
    pub async fn get_by_game_id(conn: &PgConnection, id: i32) -> Result<Vec<Round>, diesel::result::Error> {
        Ok(conn.run(move |c| rounds::table.filter(rounds::game_id.eq(id)).load::<Round>(c)).await?)
    }

    ///
    /// update round
    /// - `conn`: [PgConnection]
    /// - `id`: [i32]
    /// - `update_round`: [UpdateRound]
    /// - Returns: [Round, diesel::result::Error]
    ///
    pub async fn update(conn: &PgConnection, id: i32, update_round: Json<UpdateRound>) -> Result<Round, diesel::result::Error> {
        Ok(conn.run(move |c| diesel::update(rounds::table.filter(rounds::id.eq(id))).set(update_round.into_inner()).get_result(c)).await?)
    }

    ///
    /// delete round
    /// - `conn`: [PgConnection]
    /// - `id`: [i32]
    /// - Returns: [diesel::result::Error]
    ///
    pub async fn delete(conn: &PgConnection, id: i32) -> Result<usize, diesel::result::Error> {
        Ok(conn.run(move |c| diesel::delete(users::table.find(id)).execute(c)).await?)
    }
}
