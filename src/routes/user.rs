use diesel::prelude::*;
use rocket::{
    response::status::{Created, NoContent, NotFound},
    serde::json::Json,
    routes,
    fairing::AdHoc,
};
use crate::{
    models::{NewUser, User, UpdatedUser},
    schema::users,
    ApiError,
    PgConnection,
};

#[rocket::get("/")]
async fn list(connection: PgConnection) -> Result<Json<Vec<User>>, NotFound<Json<ApiError>>> {
    match User::all(&connection).await {
        Ok(users) => Ok(Json(users)),
        Err(e) => Err(NotFound(Json(ApiError {
            details: e.to_string(),
        }))),
    }
}


#[rocket::get("/<id>")]
async fn retrieve(connection: PgConnection, id: i32) -> Result<Json<User>, NotFound<Json<ApiError>>> {
    match User::show(id, &connection).await {
        Ok(user) => Ok(Json(user)),
        Err(e) => Err(NotFound(Json(ApiError {
            details: e.to_string(),
        })))
    }
}

#[rocket::post("/", data = "<user>")]
async fn create(connection: PgConnection, user: Json<NewUser>) -> Result<Created<Json<User>>, Json<ApiError>> {
    match User::create(user, &connection).await {
        Ok(user) => Ok(Created::new("/").body(Json(user))),
        Err(e) => Err(Json(ApiError {
            details: e.to_string(),
        }))
    }
}

#[rocket::patch("/<id>", data = "<user>")]
async fn update(connection: PgConnection, id: i32, user: Json<UpdatedUser>) -> Result<Json<User>, NotFound<Json<ApiError>>> {
    match User::update(id, user, &connection).await {
        Ok(user) => Ok(Json(user)),
        Err(e) => Err(NotFound(Json(ApiError {
            details: e.to_string(),
        })))
    }
}

#[rocket::delete("/<id>")]
async fn destroy(connection: PgConnection, id: i32) -> Result<NoContent, NotFound<Json<ApiError>>> {
    connection
        .run(move |c| {
            let affected = diesel::delete(users::table.filter(users::id.eq(id)))
                .execute(c)
                .expect("Connection is broken");
            match affected {
                1 => Ok(()),
                0 => Err("NotFound"),
                _ => Err("???"),
            }
        })
        .await
        .map(|_| NoContent)
        .map_err(|e| {
            NotFound(Json(ApiError {
                details: e.to_string(),
            }))
        })
}


pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Users api", |rocket| async {
        return rocket.mount("/user", routes![list, retrieve, create, update, destroy]);
    })
}