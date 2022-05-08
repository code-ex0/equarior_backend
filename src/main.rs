use equarior::routes::*;
use equarior::PgConnection;

#[rocket::launch]
fn rocket() -> _ {
    rocket::build()
        .attach(PgConnection::fairing())
        .attach(user::stage())
        .attach(auth::stage())
}
