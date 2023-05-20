use rocket::serde::json::{to_string, Json};
use rocket::serde::Serialize;
#[derive(Serialize)]
pub struct User {
    pub id: i32,
    pub name: &'static str,
    pub age: i32,
}

fn get_users() -> &'static [User] {
    &[
        User {
            id: 1,
            name: "Jagad Yudha",
            age: 23,
        },
        User {
            id: 2,
            name: "Bambang Andi",
            age: 27,
        },
    ]
}

#[get("/")]
pub fn users() -> Json<&'static [User]> {
    Json(get_users())
}

#[get("/<id>")]
pub fn user(id: i32) -> Option<Json<&'static User>> {
    get_users()
        .iter()
        .find(|x: &&User| x.id == id)
        .map(|user: &User| Json(user))
}
