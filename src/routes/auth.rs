use std::{collections::HashSet, sync::RwLock};

use actix_http::{
    error::InternalError,
    http::{
        header::{self, ContentType},
        HeaderValue,
    },
};
use actix_web::{
    dev::ServiceRequest,
    get, post,
    web::{self, Data, Json, Payload},
    FromRequest, HttpRequest, HttpResponse, Responder,
};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use rand::{distributions::Alphanumeric, Rng};
use serde::{Deserialize, Serialize};
use sqlx::{query, query_as, FromRow, PgPool};
use uuid::Uuid;

pub async fn auth_middleware(
    req: ServiceRequest,
    b: BearerAuth,
) -> Result<ServiceRequest, actix_http::Error> {
    if req
        .app_data::<Data<AuthState>>()
        .unwrap()
        .sessions
        .read()
        .unwrap()
        .contains(b.token())
    {
        Ok(req)
    } else {
        let mut err = HttpResponse::Ok()
            .set(ContentType::json())
            .body("{\"success\": false}");
        err.headers_mut()
            .append(header::WWW_AUTHENTICATE, HeaderValue::from_static("Bearer"));
        let err = InternalError::from_response("Failed to authenticate user", err);
        Err(err.into())
    }
}

#[derive(Debug, Default)]
pub struct AuthState {
    sessions: RwLock<HashSet<String>>,
}

#[get("/")]
async fn root() -> impl Responder {
    HttpResponse::Ok().body("Salut ça taff ? Auth part")
}

#[derive(Debug, FromRow, Serialize, Deserialize, PartialEq, Eq)]
struct RegisterInformations {
    email: String,
    username: String,
    password: String,
    school: String,
}

#[post("/register")]
async fn register(
    req: Json<RegisterInformations>,
    db: Data<PgPool>,
    auth_state: Data<AuthState>,
) -> impl Responder {
    let mut config = argon2::Config::default();
    config.variant = argon2::Variant::Argon2id;
    let salt: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(128)
        .map(char::from)
        .collect();
    let hash = argon2::hash_encoded(req.password.as_bytes(), salt.as_bytes(), &config).unwrap();

    match query(
        "INSERT INTO users (email, username, salt, password, school) VALUES ($1, $2, $3, $4, $5)",
    )
    .bind(&req.email)
    .bind(&req.username)
    .bind(salt)
    .bind(hash)
    .bind(&req.school)
    .execute(db.as_ref())
    .await
    {
        Ok(_) => {
            let token = Uuid::new_v4().to_hyphenated().to_string();
            auth_state.sessions.write().unwrap().insert(token.clone());
            HttpResponse::Ok()
                .set(ContentType::json())
                .body(format!("{{\"success\": true, \"token\": \"{}\"}}", token))
        }
        Err(err) => {
            println!("{:?}", err);
            HttpResponse::Ok()
                .set(ContentType::json())
                .body("{\"success\": false}")
        }
    }
}

#[derive(Debug, FromRow, Serialize, Deserialize, PartialEq, Eq)]
struct Credentials {
    email: String,
    password: String,
}

#[post("/login")]
async fn login(
    req: Json<Credentials>,
    db: Data<PgPool>,
    auth_state: Data<AuthState>,
) -> impl Responder {
    #[derive(FromRow)]
    struct User {
        salt: String,
        password: String,
    }

    let user: User = query_as("SELECT salt, password FROM users WHERE email = $1")
        .bind(&req.email)
        .fetch_one(db.as_ref())
        .await
        .unwrap();

    let mut config = argon2::Config::default();
    config.variant = argon2::Variant::Argon2id;
    let hash =
        argon2::hash_encoded(req.password.as_bytes(), user.salt.as_bytes(), &config).unwrap();

    if hash == user.password {
        let token = Uuid::new_v4().to_hyphenated().to_string();
        auth_state.sessions.write().unwrap().insert(token.clone());
        HttpResponse::Ok()
            .set(ContentType::json())
            .body(format!("{{\"success\": true, \"token\": \"{}\"}}", token))
    } else {
        HttpResponse::Ok()
            .set(ContentType::json())
            .body("{\"success\": false}")
    }
}

#[post("/logout")]
async fn logout(req: HttpRequest, payload: Payload, auth_state: Data<AuthState>) -> impl Responder {
    if let Ok(auth) = BearerAuth::from_request(&req, &mut payload.into_inner()).await {
        if auth_state.sessions.write().unwrap().remove(auth.token()) {
            HttpResponse::Ok()
                .set(ContentType::json())
                .body("{\"success\": true}")
        } else {
            HttpResponse::Ok()
                .set(ContentType::json())
                .body("{\"success\": false}")
        }
    } else {
        HttpResponse::Ok()
            .set(ContentType::json())
            .body("{\"success\": false}")
    }
}

pub fn init(cfg: &mut web::ServiceConfig, auth_state: web::Data<AuthState>) {
    cfg.app_data(auth_state)
        .service(root)
        .service(register)
        .service(login)
        .service(logout);
}
