use rocket::serde::{Deserialize, Serialize};
use diesel::{
    prelude::*,
    AsChangeset,
    Insertable,
    Queryable
};
use rocket::serde::json::Json;
use crate::{
    schema::games,
    PgConnection
};



///
/// Game model
/// used to store user information
///
#[derive(Serialize, Queryable, Debug)]
#[serde(crate = "rocket::serde")]
pub struct Game {
    pub id: i32,
    pub ids_players: Vec<i32>,
    pub created_at: chrono::NaiveDateTime,
}


///
/// game model
/// used to insert new game
///
#[derive(Serialize, Deserialize, Insertable, Debug)]
#[serde(crate = "rocket::serde")]
#[table_name = "games"]
pub struct NewGame {
    pub ids_players: Vec<i32>,
}

///
/// game model
/// used to update game
///
#[derive(Serialize, Deserialize, AsChangeset, Debug)]
#[serde(crate = "rocket::serde")]
#[table_name = "games"]
pub struct UpdateGame {
    pub ids_players: Vec<i32>,
}


impl Game {

    ///
    /// Get game by id
    /// - `id`: game id
    /// - `conn`: [PgConnection]
    /// - Returns: [Game, diesel::result::Error]
    ///
    pub async fn get_by_id(conn: &PgConnection, id: i32) -> Result<Game, diesel::result::Error> {
        Ok(conn.run(move |c| games::table.filter(games::id.eq(id)).first(c)).await?)
    }
    ///
    /// get all games
    /// - `conn`: [PgConnection]
    /// - Returns: [Vec<Game>, diesel::result::Error]
    ///
    pub async fn get_all(conn: &PgConnection) -> Result<Vec<Game>, diesel::result::Error> {
        Ok(conn.run(move |c| games::table.load(c)).await?)
    }


    ///
    /// insert new game
    /// - `new_game`: [NewGame]
    /// - `conn`: [PgConnection]
    /// - Returns: [Game, diesel::result::Error]
    ///
    pub async fn insert(conn: &PgConnection, new_game: Json<NewGame>) -> Result<Game, diesel::result::Error> {
        Ok(conn.run(move |c| diesel::insert_into(games::table).values(new_game.into_inner()).get_result(c)).await?)
    }

    ///
    /// update game
    /// - `id`: game id
    /// - `conn`: [PgConnection]
    /// - `new_game`: [UpdateGame]
    /// - Returns: [Game, diesel::result::Error]
    ///
    pub async fn update(conn: &PgConnection, id: i32, new_game: Json<UpdateGame>) -> Result<Game, diesel::result::Error> {
        Ok(conn.run(move |c| diesel::update(games::table.filter(games::id.eq(id))).set(new_game.into_inner()).get_result(c)).await?)
    }
}

