use actix_web::{web, HttpResponse, Responder};
use crate::models::user::{User, CreateUser};
use crate::db::DbPool;
use uuid::Uuid;
use log::error;

pub async fn create_user(pool: web::Data<DbPool>, user: web::Json<CreateUser>) -> impl Responder {
    let mut conn = match pool.get() {
        Ok(conn) => conn,
        Err(e) => {
            error!("Failed to get connection from pool: {:?}", e);
            return HttpResponse::InternalServerError().json("Failed to get connection from pool");
        }
    };

    println!("Creating user: {:?}", user);

    match User::create(web::Json(user).into_inner(), &mut conn) {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(e) => {
            error!("Failed to create user: {:?}", e);
            HttpResponse::InternalServerError().json("Failed to create user")
        },
    }
}

pub async fn get_users(pool: web::Data<DbPool>) -> impl Responder {
    let mut conn = pool.get().expect("Failed to get connection from pool");

    match User::find_all(&mut conn) {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn get_user(pool: web::Data<DbPool>, user_id: web::Path<Uuid>) -> impl Responder {
    let mut conn = pool.get().expect("Failed to get connection from pool");

    match User::find_by_id(user_id.into_inner(), &mut conn) {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn update_user(pool: web::Data<DbPool>, user_id: web::Path<Uuid>, user: web::Json<CreateUser>) -> impl Responder {
    let mut conn = match pool.get() {
        Ok(conn) => conn,
        Err(e) => {
            error!("Failed to get connection from pool: {:?}", e);
            return HttpResponse::InternalServerError().json("Failed to get connection from pool");
        }
    };

    match User::update(user_id.into_inner(), user, &mut conn) {
        Ok(updated_user) => HttpResponse::Ok().json(updated_user),
        Err(e) => {
            error!("Failed to update user: {:?}", e);
            HttpResponse::InternalServerError().json("Failed to update user")
        },
    }
}

pub async fn delete_user(pool: web::Data<DbPool>, user_id: web::Path<Uuid>) -> impl Responder {
    let mut conn = match pool.get() {
        Ok(conn) => conn,
        Err(e) => {
            error!("Failed to get connection from pool: {:?}", e);
            return HttpResponse::InternalServerError().json("Failed to get connection from pool");
        }
    };

    match User::delete(user_id.into_inner(), &mut conn) {
        Ok(deleted_count) => {
            if deleted_count > 0 {
                HttpResponse::Ok().json("User deleted successfully")
            } else {
                HttpResponse::NotFound().json("User not found")
            }
        },
        Err(e) => {
            error!("Failed to delete user: {:?}", e);
            HttpResponse::InternalServerError().json("Failed to delete user")
        },
    }
}