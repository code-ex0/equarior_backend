use rocket::{
    response::status::NotFound,
    serde::json::Json,
    routes,
    fairing::AdHoc,
};
use crate::{
    ApiError,
    PgConnection,
};
use crate::jwt::ApiKey;
use crate::models::{NewRound, Round};


///
/// get all rounds
///
#[rocket::get("/")]
pub async fn get_rounds(_apikey: ApiKey, conn: PgConnection) -> Result<Json<Vec<Round>>, NotFound<Json<ApiError>>> {
    match Round::get_all(&conn).await {
        Ok(rounds) => Ok(Json(rounds)),
        Err(e) => Err(NotFound(Json(ApiError::new(e.to_string())))),
    }
}

///
/// get a round by id
///
#[rocket::get("/<id>")]
pub async fn get_round(_apikey: ApiKey, id: i32, conn: PgConnection) -> Result<Json<Round>, NotFound<Json<ApiError>>> {
    match Round::get_by_id(&conn, id).await {
        Ok(round) => Ok(Json(round)),
        Err(e) => Err(NotFound(Json(ApiError::new(e.to_string())))),
    }
}

///
/// create a new round
///
#[rocket::post("/", data = "<round>")]
pub async fn create_round(_apikey: ApiKey, round: Json<NewRound>, conn: PgConnection) -> Result<Json<Round>, Json<ApiError>> {
    match Round::insert(&conn, round).await {
        Ok(round) => Ok(Json(round)),
        Err(e) => Err(Json(ApiError::new(e.to_string()))),
    }
}


pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Game api", |rocket| async {
        return rocket.mount("/round", routes![get_rounds, get_round, create_round]);
    })
}
