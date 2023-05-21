#[macro_use]
extern crate rocket;

mod database;
pub mod routes;

#[launch]
fn rocket() -> _ {
    rocket::build().mount(
        "/users",
        routes![
            routes::users::users,
            routes::users::user,
            routes::users::new_user,
            routes::users::edit_user
        ],
    )
}
