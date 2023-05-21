use diesel::prelude::*;
use models::user::{EditUser, NewUser, User};
use rocket_diesel_boilerplate::*;

pub fn find_all(conn: &mut PgConnection) -> Vec<User> {
    use self::schema::users::dsl::*;
    users.load::<User>(conn).expect("Error loading users")
}

pub fn find(conn: &mut PgConnection, user_id: &i32) -> Option<User> {
    use self::schema::users::dsl::*;
    users
        .find(user_id)
        .limit(1)
        .first(conn)
        .optional()
        .expect("Error loading user")
}

pub fn new(conn: &mut PgConnection, name: &str, avatar: &str) -> User {
    use schema::users;
    let new_user: NewUser = NewUser { name, avatar };
    diesel::insert_into(users::table)
        .values(&new_user)
        .get_result(conn)
        .expect("Error adding user")
}

pub fn edit(conn: &mut PgConnection, id: &i32, name: &str, avatar: &str) -> User {
    use schema::users;
    let edit_user: EditUser = EditUser { name, avatar };
    diesel::update(users::table.find(id))
        .set(&edit_user)
        .get_result(conn)
        .expect("Error loading user")
}
