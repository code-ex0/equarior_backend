use rocket::serde::{Deserialize, Serialize};
use diesel::{
    prelude::*,
    AsChangeset,
    Insertable,
    Queryable
};
use rocket::serde::json::Json;
use crate::{
    schema::users,
    PgConnection
};

#[derive(Serialize, Queryable, Debug)]
#[serde(crate = "rocket::serde")]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    #[serde(skip_serializing)]
    pub password: String,
    #[serde(skip_serializing)]
    pub update_at: chrono::NaiveDateTime,
    #[serde(skip_serializing)]
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Serialize, Deserialize, Debug, Queryable)]
#[serde(crate = "rocket::serde")]
pub struct LoginUser {
    pub email: String,
    pub password: String
}

#[derive(Deserialize, Insertable, Debug)]
#[serde(crate = "rocket::serde")]
#[table_name = "users"]
pub struct NewUser {
    pub username: String,
    pub email: String,
    pub password: String
}

#[derive(Deserialize, AsChangeset, Debug)]
#[serde(crate = "rocket::serde")]
#[table_name = "users"]
pub struct UpdatedUser {
    pub username: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
}

impl User {
    pub async fn show(id: i32, conn: &PgConnection) -> Result<User, diesel::result::Error> {
        Ok(conn.run(move |c| users::table.filter(users::id.eq(id)).first(c)).await?)
    }

    pub async fn all(conn: &PgConnection) -> Result<Vec<User>, diesel::result::Error> {
        Ok(conn.run(move |c| users::table.load(c)).await?)
    }

    pub async fn create(new_user: Json<NewUser>, conn: &PgConnection) -> Result<User, diesel::result::Error> {
        Ok(conn.run(move |c| diesel::insert_into(users::table).values(&new_user.into_inner()).get_result(c)).await?)
    }

    pub async fn update(id: i32, updated_user:  Json<UpdatedUser>, conn: &PgConnection) -> Result<User, diesel::result::Error> {
        Ok(conn.run(move |c| diesel::update(users::table.find(id)).set(&updated_user.into_inner()).get_result(c)).await?)
    }

    pub async fn delete(id: i32, conn: &PgConnection) -> Result<usize, diesel::result::Error> {
        Ok(conn.run(move |c| diesel::delete(users::table.find(id)).execute(c)).await?)
    }

    pub async fn login(login_user: Json<LoginUser>, conn: &PgConnection) -> Result<User, diesel::result::Error> {
        Ok(conn.run(move |c| users::table.filter(users::email.eq(&login_user.into_inner().email)).first(c)).await?)
    }
}