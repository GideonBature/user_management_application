use uuid::Uuid;
use diesel::prelude::*;
use chrono::prelude::*;
use crate::schema::users;
use serde::{Deserialize, Serialize};
use actix_web::web::{self};



#[derive(Debug, Queryable, Selectable, Serialize, Deserialize, Insertable)]
#[diesel(table_name = users)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}



#[derive(Debug, Deserialize)]
pub struct CreateUser {
    pub username: String,
    pub email: String,
}

impl User {
    pub fn create(user: web::Json<CreateUser>, conn: &mut PgConnection) -> QueryResult<User> {

        let new_user = User {
            id: Uuid::new_v4(),
            username: user.username.clone(),
            email: user.email.clone(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        diesel::insert_into(users::table).values(&new_user)
            .get_result::<User>(conn)
    }

    pub fn find_all(conn: &mut PgConnection) -> QueryResult<Vec<User>> {
        users::table.load::<User>(conn)
    }

    pub fn find_by_id(id: Uuid, conn: &mut PgConnection) -> QueryResult<User> {
        users::table.find(id).get_result::<User>(conn)
    }

    pub fn update(id: Uuid, user: web::Json<CreateUser>, conn: &mut PgConnection) -> QueryResult<User> {
        diesel::update(users::table.find(id))
            .set((
                users::username.eq(&user.username),
                users::email.eq(&user.email),
                users::updated_at.eq(Utc::now()),
            ))
            .get_result::<User>(conn)
    }

    pub fn delete(id: Uuid, conn: &mut PgConnection) -> QueryResult<usize> {
        diesel::delete(users::table.find(id)).execute(conn)
    }
}
