mod models;

use actix_web::{get, patch, post, App, HttpResponse, HttpServer, Responder};
use actix_web::web::Json;
use validator::Validate;
use crate::models::BuyPizzaRequest;

#[get("/pizzas")]
async fn get_pizzas() -> impl Responder {
    HttpResponse::Ok().body("Pizzas Available ")
}

#[post("/buypizza")]
async fn buy_pizza(body: Json<BuyPizzaRequest>) -> impl Responder {
    let is_valid = body.validate();
    return match is_valid {
        Ok(_) => {
            let pizza_name = body.pizza_name.clone();
            HttpResponse::Ok().body("Pizza entered is {}", pizza_name)
        },
        Err(_) => {
            return HttpResponse::Ok().body("Pizza name is required!")
        }
    }
}

#[patch("/buypizza/{uuid}")]
async fn update_pizza() -> impl Responder {
    HttpResponse::Ok().body("Updating a Pizza!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(get_pizzas)
            .service(buy_pizza)
            .service(update_pizza)
    })
    .bind("127.0.0.1:8001")?
    .run()
    .await
}
