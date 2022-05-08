use rocket::routes;
use rocket::fairing::AdHoc;
use rocket::{
    response::status::NotFound,
    serde::json::Json,
};
use crate::{
    models::{NewUser, User},
    ApiError,
    PgConnection,
};
use crate::models::user::LoginUser;

#[rocket::post("/register", data="<user>")]
async fn register(connection: PgConnection, user: Json<NewUser>) -> Result<Json<User>, NotFound<Json<ApiError>>> {
    let mut user = user.into_inner();
    user.password = bcrypt::hash(&user.password, bcrypt::DEFAULT_COST).unwrap();
    match User::create(user.into(), &connection).await {
        Ok(user) => Ok(Json(user)),
        Err(e) => Err(NotFound(Json(ApiError::new(e.to_string())))),
    }
}

#[rocket::post("/login", data="<_user>")]
async fn login(_connection: PgConnection, _user: Json<LoginUser>) -> Result<Json<User>, NotFound<Json<ApiError>>> {
    todo!()
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Auth api", |rocket| async {
        return rocket.mount("/auth", routes![register, login]);
    })
}