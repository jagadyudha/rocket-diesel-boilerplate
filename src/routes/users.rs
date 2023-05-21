use diesel::prelude::*;
use models::user::NewUser;
use models::user::User;
use rocket::form::Form;
use rocket::serde::json::Json;
use rocket_diesel_boilerplate::*;

#[get("/")]
pub fn users() -> Json<Vec<User>> {
    use crate::database::users::find_all;
    let connection: &mut PgConnection = &mut establish_connection();
    let results = find_all(connection);
    Json(results)
}

#[get("/<user_id>")]
pub fn user(user_id: i32) -> Option<Json<User>> {
    use crate::database::users::find;
    let connection: &mut PgConnection = &mut establish_connection();
    let results = find(connection, &user_id);
    results.map(Json)
}

#[post("/", data = "<user_form>")]
pub fn new_user(user_form: Form<NewUser>) -> Json<User> {
    use crate::database::users::new;
    let connection: &mut PgConnection = &mut establish_connection();
    let results = new(connection, &user_form.name, &user_form.avatar);
    Json(results)
}

#[patch("/<user_id>", data = "<user_form>")]
pub fn edit_user(user_id: i32, user_form: Form<NewUser>) -> Json<User> {
    use crate::database::users::edit;
    let connection: &mut PgConnection = &mut establish_connection();
    let results = edit(connection, &user_id, &user_form.name, &user_form.avatar);
    Json(results)
}
