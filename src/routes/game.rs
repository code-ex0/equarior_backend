use rocket::fairing::AdHoc;
use rocket::routes;
use rocket::response::status::NotFound;
use rocket::serde::json::Json;
use crate::jwt::{ApiKey, read_token};
use crate::models::game::{Game, NewGame};
use crate::{ApiError, PgConnection};
use crate::models::UpdateGame;

///
/// just a test route
///
#[rocket::get("/hello")]
pub fn index(_api: ApiKey, _conn: PgConnection) -> &'static str {
    "Hello, world!"
}

///
/// new game
///
#[rocket::post("/", data = "<game>")]
pub async fn new_game(_api: ApiKey, conn: PgConnection, game: Json<NewGame>) -> Result<Json<Game>, NotFound<Json<ApiError>>> {
   match Game::insert(&conn, game).await {
       Ok(game) => Ok(Json(game)),
       Err(e) => Err(NotFound(Json(ApiError::new(e.to_string())))),
   }
}

///
/// get game
///
#[rocket::get("/<id>")]
pub async fn get_game(_api: ApiKey, conn: PgConnection, id: i32) ->  Result<Json<Game>, NotFound<Json<ApiError>>> {
    match Game::get_by_id(&conn, id).await {
        Ok(game) => Ok(Json(game)),
        Err(err) => Err(NotFound(Json(ApiError::new(err.to_string())))),
    }
}

///
/// get all games
///
#[rocket::get("/")]
pub async fn get_all_games(_api: ApiKey, conn: PgConnection) ->  Result<Json<Vec<Game>>, NotFound<Json<ApiError>>> {
    match Game::get_all(&conn).await {
        Ok(games) => Ok(Json(games)),
        Err(err) => Err(NotFound(Json(ApiError::new(err.to_string()))))
    }
}

///
/// create game
///
#[rocket::post("/<token>")]
pub async fn create_game(_api: ApiKey, conn: PgConnection, token: String) -> Result<Json<Game>, NotFound<Json<ApiError>>> {
    let data = read_token(&token).unwrap();
    let game = NewGame {
        ids_players: vec![data.parse::<i32>().unwrap()],
    };
    match Game::insert(&conn, Json(game)).await {
        Ok(game) => Ok(Json(game)),
        Err(e) => Err(NotFound(Json(ApiError::new(e.to_string())))),
    }
}


///
/// join game
///
#[rocket::post("/<token>/<id>")]
pub async fn join_game(_api: ApiKey, conn: PgConnection, token: String, id: i32) -> Result<Json<Game>, NotFound<Json<ApiError>>> {
    let data = read_token(&token).unwrap();
    let mut game = Game::get_by_id(&conn, id).await.unwrap();
    let id_player = data.parse::<i32>().unwrap();
    if game.ids_players.contains(&id_player) {
        return Err(NotFound(Json(ApiError::new("Player already in game".to_string()))));
    }
    game.ids_players.push(id_player);
    let update_game = UpdateGame {
        ids_players: game.ids_players.clone(),
    };
    match Game::update(&conn, id, Json(update_game)).await {
        Ok(game) => Ok(Json(game)),
        Err(e) => Err(NotFound(Json(ApiError::new(e.to_string())))),
    }
}


pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Game api", |rocket| async {
        return rocket.mount("/game", routes![index, new_game, get_game, get_all_games, create_game, join_game]);
    })
}
