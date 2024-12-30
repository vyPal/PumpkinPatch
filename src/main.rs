use std::sync::Arc;

use actix_cors::Cors;
use actix_session::{storage::CookieSessionStore, Session, SessionExt, SessionMiddleware};
use actix_web::{body::MessageBody, cookie::{self, Cookie, Key, SameSite}, dev::{Service, ServiceRequest, ServiceResponse}, get, middleware::Next, post, web, App, HttpResponse, HttpServer, Responder};
use diesel::{r2d2::{ConnectionManager, Pool}, ExpressionMethods, OptionalExtension, PgConnection, QueryDsl, RunQueryDsl};
use minio::s3::client;
use models::{NewUser, User};
use oauth2::{basic::BasicClient, reqwest::async_http_client, AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, RedirectUrl, TokenResponse, TokenUrl};
use openid::{Client, Discovered, DiscoveredClient, Options, StandardClaims, Token, Userinfo};
use schema::users::access_token;
use serde::{Deserialize, Serialize};

mod schema;
mod models;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[derive(Clone, Debug)]
struct OidcConfig {
    client_id: String,
    client_secret: String,
    issuer_url: String,
    redirect_url: String,
}

impl OidcConfig {
    pub fn from_env() -> Self {
        Self {
            client_id: check_env_var("OIDC_CLIENT_ID"),
            client_secret: check_env_var("OIDC_CLIENT_SECRET"),
            issuer_url: check_env_var("OIDC_ISSUER_URL"),
            redirect_url: check_env_var("OIDC_REDIRECT_URL")
        }
    }
}

// Initialize login
#[get("/auth/login")]
async fn github_login(
    client: web::Data<Arc<DiscoveredClient>>,
) -> impl Responder {
    let origin_url = check_env_var("ORIGIN");
    let auth_url = client
        .auth_url(&Options{
            scope: Some("openid profile email".to_string()),
            state: Some(origin_url),
            ..Default::default()
        });

    HttpResponse::Found()
        .append_header(("Location", auth_url.to_string()))
        .finish()
}

// Handle the OAuth callback
#[derive(Deserialize)]
struct AuthCallback {
    code: String,
}

async fn request_token(
    oidc_client: &DiscoveredClient,
    login_query: &AuthCallback,
) -> Result<Option<(Token, Userinfo)>, openid::error::Error> {
    let mut token: Token = oidc_client.request_token(&login_query.code).await?.into();

    if let Some(id_token) = token.id_token.as_mut() {
        oidc_client.decode_token(id_token)?;
        oidc_client.validate_token(id_token, None, None)?;
        println!("token: {:?}", id_token);
    } else {
        return Ok(None);
    }

    let userinfo = oidc_client.request_userinfo(&token).await?;

    println!("user info: {:?}", userinfo);

    Ok(Some((token, userinfo)))
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserSession {
    pub user_id: String,
    pub username: String,
    pub email: String,
}

fn insert_new_user(
    conn: &mut PgConnection,
    user: NewUser,
) -> Result<usize, diesel::result::Error> {
    use crate::schema::users::dsl::*;

    // normal diesel operations
    diesel::insert_into(users)
        .values(&user)
        .execute(conn)
}

fn find_user_by_oidc_id(
    conn: &mut PgConnection,
    oidc_id_str: &str,
) -> Result<Option<User>, diesel::result::Error> {
    use crate::schema::users::dsl::*;

    users
        .filter(oidc_id.eq(oidc_id_str))
        .first(conn)
        .optional()
}

#[get("/auth/callback")]
async fn auth_callback(
    query: web::Query<AuthCallback>,
    oidc_client: web::Data<Arc<DiscoveredClient>>,
    session: Session,
    db: web::Data<Pool<ConnectionManager<PgConnection>>>,
) -> impl Responder {
    let request_token = request_token(&oidc_client, &query).await;
    match request_token {
        Ok(Some((token, user_info))) => {
            let login = user_info.preferred_username.clone();
            let email = user_info.email.clone();

            let origin = check_env_var("ORIGIN");
            let at = token.bearer.access_token.clone();

            // Since some IDPs use email as the preferred username, we need to handle that
            let mut login_str = String::new();
            if let Some(login) = login {
                if login.contains("@") {
                    let split: Vec<&str> = login.split("@").collect();
                    login_str = split[0].to_string();
                } else {
                    login_str = login.to_string();
                }
            }

            let user_result = find_user_by_oidc_id(&mut db.get().expect("couldn't get db connection from pool"), &user_info.sub.clone().expect("No OIDC ID found"));

            let user_session = if user_result.is_err() || (user_result.as_ref().ok().into_iter().flatten().next().is_none()) {
                let user = NewUser {
                    username: login_str.clone(),
                    oidc_id: user_info.sub.clone().expect("No OIDC ID found"),
                    email: email.clone().expect("No email found"),
                    access_token: token.bearer.access_token.clone(),
                    refresh_token: token.bearer.refresh_token.clone(),
                };
    
                let _ = web::block(move || {
                    let mut conn = db.get().expect("couldn't get db connection from pool");
            
                    insert_new_user(&mut conn, user)
                })
                .await
                .map_err(actix_web::error::ErrorInternalServerError);

                UserSession {
                    user_id: user_info.sub.expect("No OIDC ID found"),
                    username: login_str,
                    email: email.expect("No email found"),
                }
            } else if let Ok(Some(user)) = user_result {
                let username = user.username.clone();
                let _ = web::block(move || {
                    let mut conn = db.get().expect("couldn't get db connection from pool");
            
                    diesel::update(&user)
                        .set((
                            schema::users::access_token.eq(token.bearer.access_token.clone()),
                            schema::users::refresh_token.eq(token.bearer.refresh_token.clone()),
                        ))
                        .execute(&mut conn)
                })
                .await
                .map_err(actix_web::error::ErrorInternalServerError);

                UserSession {
                    user_id: user_info.sub.expect("No OIDC ID found"),
                    username: username,
                    email: email.expect("No email found"),
                }
            } else {
                UserSession {
                    user_id: user_info.sub.expect("No OIDC ID found"),
                    username: login_str,
                    email: email.expect("No email found"),
                }
            };
            
            let _ = session.insert("user", user_session)
                .map_err(|_| actix_web::error::ErrorInternalServerError("Session error"));

            HttpResponse::Found()
                .cookie(
                    Cookie::build("auth_token", at)
                        .http_only(true)
                        .secure(true)
                        .same_site(SameSite::Lax)
                        .finish()
                )
                .append_header(("Location", origin+"/dashboard"))
                .finish()
        }
        Ok(None) => {
            println!("login error in call: no id_token found");

            HttpResponse::Unauthorized().finish()
        }
        Err(err) => {
            println!("login error in call: {:?}", err);

            HttpResponse::Unauthorized().finish()
        }
    }
}

#[get("/api/user")]
async fn get_user(session: Session) -> impl Responder {
    if let Ok(Some(user)) = session.get::<UserSession>("user") {
        HttpResponse::Ok().json(user)
    } else {
        HttpResponse::Unauthorized().finish()
    }
}

#[post("/api/logout")]
async fn logout(session: Session) -> impl Responder {
    session.purge();
    HttpResponse::Ok().finish()
}

pub async fn auth_middleware(
    req: ServiceRequest,
    srv: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, actix_web::error::Error> {
    let session = req.get_session();
    if session.get::<UserSession>("user").ok().flatten().is_some() {
        return srv.call(req).await;
    }
    Err(actix_web::error::ErrorUnauthorized("Unauthorized"))
}

fn check_env_var(name: &str) -> String {
    std::env::var(name).expect(&format!("Missing {}", name))
}

fn check_env() {
    check_env_var("DATABASE_URL");
    check_env_var("OIDC_CLIENT_ID");
    check_env_var("OIDC_CLIENT_SECRET");
    check_env_var("OIDC_ISSUER_URL");
    check_env_var("OIDC_REDIRECT_URL");
    check_env_var("ORIGIN");
    check_env_var("SECRET_KEY");
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();
    check_env();

    let config = OidcConfig::from_env();

    let secret_key = Key::from(check_env_var("SECRET_KEY").as_bytes());

    let client = Arc::new(DiscoveredClient::discover(config.client_id, config.client_secret, config.redirect_url, reqwest::Url::parse(&config.issuer_url).expect("Failed to parse ISSUER_URL")).await.unwrap());

    let database_url = check_env_var("DATABASE_URL");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = Pool::builder().build(manager).expect("Failed to create pool");

    HttpServer::new(move || {
        App::new()
            .wrap(Cors::permissive())
            .wrap(
                SessionMiddleware::builder(CookieSessionStore::default(), secret_key.clone())
                    .cookie_secure(false)
                    .cookie_http_only(false)
                    .build()
            )
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(client.clone()))
            .service(hello)
            .service(echo)
            .service(github_login)
            .service(auth_callback)
            .service(logout)
            .service(get_user)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
