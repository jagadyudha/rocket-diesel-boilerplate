use diesel::prelude::*;
use models::user::User;
use rocket::serde::json::Json;
use rocket_diesel_boilerplate::*;

#[get("/")]
pub fn users() -> Json<Vec<User>> {
    use self::schema::users::dsl::*;
    let connection: &mut PgConnection = &mut establish_connection();
    let results: Vec<User> = users.load::<User>(connection).expect("Error loading users");
    Json(results)
}

#[get("/<user_id>")]
pub fn user(user_id: i32) -> Json<Vec<User>> {
    use self::schema::users::dsl::*;
    let connection: &mut PgConnection = &mut establish_connection();
    let results: Vec<User> = users
        .find(user_id)
        .load::<User>(connection)
        .expect("Error loading user");
    Json(results)
}
