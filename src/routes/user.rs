use hmac::{Hmac, Mac};
use sha2::Sha256;
use jwt::{RegisteredClaims, SignWithKey};
use rocket::{
    response::status::{Created, NoContent, NotFound},
    serde::json::Json,
    routes,
    fairing::AdHoc,
};
use rocket::serde::json::serde_json::json;
use rocket::serde::json::Value;
use crate::{
    models::{NewUser, User, UpdatedUser},
    ApiError,
    PgConnection,
};
use crate::jwt::ApiKey;

use crate::models::user::LoginUser;

///
/// Get all users
/// - GET /user
/// - Returns: Vec<[User]>
///
#[rocket::get("/")]
async fn list(_key: ApiKey, connection: PgConnection) -> Result<Json<Vec<User>>, NotFound<Json<ApiError>>> {
    match User::get_all(&connection).await {
        Ok(users) => Ok(Json(users)),
        Err(e) => Err(NotFound(Json(ApiError {
            details: e.to_string(),
        }))),
    }
}

///
/// Get a user by id
/// - GET /user/:id
/// - Returns: [User]
///
#[rocket::get("/<id>")]
async fn retrieve(_key: ApiKey, connection: PgConnection, id: i32) -> Result<Json<User>, NotFound<Json<ApiError>>> {
    match User::get_by_id(id, &connection).await {
        Ok(user) => Ok(Json(user)),
        Err(e) => Err(NotFound(Json(ApiError {
            details: e.to_string(),
        })))
    }
}

///
/// Creates a new user
/// - POST /user
/// - Body: [NewUser]
/// - Response: [Created]
///
#[rocket::post("/", data = "<user>")]
async fn create(_key: ApiKey, connection: PgConnection, user: Json<NewUser>) -> Result<Created<Json<User>>, Json<ApiError>> {
    match User::insert(user, &connection).await {
        Ok(user) => Ok(Created::new("/").body(Json(user))),
        Err(e) => Err(Json(ApiError {
            details: e.to_string(),
        }))
    }
}

///
/// Update a user
/// - UPDATE /users/:id
/// - Body: [UpdatedUser]
/// - Response: [NoContent]
///
#[rocket::patch("/<id>", data = "<user>")]
async fn update(_key: ApiKey, connection: PgConnection, id: i32, user: Json<UpdatedUser>) -> Result<Json<User>, NotFound<Json<ApiError>>> {
    match User::update(id, user, &connection).await {
        Ok(user) => Ok(Json(user)),
        Err(e) => Err(NotFound(Json(ApiError {
            details: e.to_string(),
        })))
    }
}

///
/// Delete
/// - DELETE /users/:id
/// - Response: [NoContent]
///
#[rocket::delete("/<id>")]
async fn destroy(_key: ApiKey, connection: PgConnection, id: i32) -> Result<NoContent, NotFound<Json<ApiError>>> {
    match User::delete(id, &connection).await {
        Ok(_) => {Ok(NoContent)}
        Err(e) => {
            Err(NotFound(Json(ApiError {
                details: e.to_string(),
            })))
        }
    }
}

///
/// Register
/// - POST /auth/register
/// - Body: [NewUser]
/// - Returns: [User]
///
#[rocket::post("/register", data="<user>")]
async fn register(connection: PgConnection, user: Json<NewUser>)
                  -> Result<Json<User>, NotFound<Json<ApiError>>>
{
    let mut user = user.into_inner();
    user.password = bcrypt::hash(&user.password, bcrypt::DEFAULT_COST).unwrap();
    match User::insert(user.into(), &connection).await {
        Ok(user) => Ok(Json(user)),
        Err(e) => Err(NotFound(Json(ApiError::new(e.to_string())))),
    }
}

///
/// Login
/// - POST /auth/login
/// - Body: [LoginUser]
/// - Returns: [User]
///
#[rocket::post("/login", data="<user>")]
async fn login(connection: PgConnection, user: Json<LoginUser>) -> Result<Json<Value>, NotFound<Json<ApiError>>> {
    let password = &user.password.clone();
    match User::login(user, &connection).await {
        Ok(valid_user) => {
            match bcrypt::verify(password, valid_user.password.as_str()) {
                Ok(_) => {
                    let claim = RegisteredClaims {
                        subject: Some(valid_user.id.to_string()),
                        ..Default::default()
                    };

                    let key: Hmac<Sha256> = Hmac::new_from_slice("secret".as_bytes()).unwrap();
                    let signed_token = claim.sign_with_key(&key).unwrap();

                    Ok(Json(json!({"token": signed_token})))
                },
                Err(_) => Err(NotFound(Json(ApiError::new("Invalid password".to_string())))),
            }
        }
        Err(e) => Err(NotFound(Json(ApiError::new(e.to_string())))),
    }
}


///
/// Logout
/// - POST /auth/logout
/// - Returns: [NoContent]
///
#[rocket::post("/logout")]
async fn logout(_connection: PgConnection, _token: ApiKey) -> Result<NoContent, NotFound<Json<ApiError>>> {
    todo!()
}

///
/// link routes to rocket
/// - GET /users
/// - GET /users/:id
/// - POST /users
/// - PATCH /users/:id
/// - DELETE /users/:id
/// - POST /auth/register
/// - POST /auth/login
/// - POST /auth/logout
/// - returns: [AdHoc]
pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Users api", |rocket| async {
        return rocket.mount("/user", routes![list, retrieve, create, update, destroy])
            .mount("/auth", routes![register, login, logout]);
    })
}