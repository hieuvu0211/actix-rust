mod libs;
mod models;
mod schema;
use crate::libs::establish_connection;
use crate::models::account::{Account, AccountResponse};
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use diesel::{QueryDsl, RunQueryDsl};

#[get("/account")]
async fn get_account() -> impl Responder {
    use self::schema::account::dsl::*;
    let mut connection = establish_connection();
    let results = web::block(move || account.limit(5).load::<Account>(&mut connection))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        });
    match results {
        Ok(Ok(accounts)) => {
            // println!("accounts: {:?}", accounts);
            HttpResponse::Ok().json(AccountResponse {accounts})
        },
        Ok(Err(_)) => HttpResponse::NotFound().finish(),
        Err(response) => response,
    }
}

#[post("/account")]
async fn create_account(data: web::Json<Account>) -> impl Responder {
    use self::schema::account::dsl::*;
    let mut connection = establish_connection();
    

    //create a new account
    let new_account = Account {
        id: 0,
        username: data.username.clone(),
        password: data.password.clone(),
        email: data.email.clone(),
        created_at: None,
        updated_at: None,
    };
    let result = web::block(move || {
        diesel::insert_into(account)
            .values(&new_account)
            .execute(&mut connection)
            
    })
    .await
    .map_err(|e| {
        eprintln!("{}", e);
        HttpResponse::InternalServerError().finish()
    })
    ;
    match result{
        Ok(_) => HttpResponse::Ok().json("user created"),
        Err(response) => response,
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port: u16 = 8080;
    HttpServer::new(|| App::new().service(get_account).service(create_account))
        .bind(("127.0.0.1", port))?
        .run()
        .await
}
