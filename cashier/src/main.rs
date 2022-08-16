use actix_web::{post, web::{self, Data}, App, HttpServer, Responder, HttpRequest};
// https://crates.io/crates/awc //https://github.com/actix/examples/blob/master/https-tls/awc-https/src/main.rs
use derive_more::{Display, Error};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Mutex;

mod key;


/* use crate::game::*; */
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let data = Data::new(Mutex::new(key::get_product()));

    // let product_data:HashMap<i32, key::Product> = key::get_product();
    HttpServer::new(move || App::new().app_data(Data::clone(&data)).service(calculate_ticket))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}

#[derive(Debug, Serialize, Deserialize)]
struct OrderCalResult {
    price: f32, 
}

#[derive(Debug, Deserialize, Clone)]
struct Order {
    // price: f32,
    // is_toy: bool,
    order: Vec<Item>,
}

#[derive(Debug, Deserialize, Clone)]
struct Item {
    id: i32,
    qty: i32,
}

#[derive(Deserialize)]
struct StatusData {
    // #[serde(rename = "status")]
    // status: i32,
    ok: bool,
}

#[post("/calculate_order")]
async fn calculate_ticket(req: HttpRequest, data: web::Json<Order>) -> impl Responder {
    // make request here
    println!("model: {:?}", &data);
    let product_data = req.app_data::<Data<Mutex<HashMap<i32, key::Product>>>>().unwrap().lock().unwrap();
    let mut order_total = 0.00;
    for elem in data.order.clone() {
        println!("{} {}", elem.id, elem.qty);
        let price_of_item = product_data[&elem.id].price;
        order_total = order_total + price_of_item * elem.qty as f32;
    }

    let telegram_api = "https://api.telegram.org/bot".to_owned()
        + key::BOT_KEY
        + "/sendMessage?chat_id="
        + key::CHAT_ID
        + "&parse_mode=Markdown&text=hello";

    let resp = reqwest::get(telegram_api).await;
    // println!("Response: {:?}", resp);

    match resp {
        Ok(data) => {
            println!("{:#?}", data);
            let json_data = data.json::<StatusData>().await.unwrap();
            println!("Response: {:?}", json_data.ok);
            /*   return web::Json(OrderCalResult {
                price: 0.0
            }); */
        }
        Err(err) => {
            println!("{:#?}", err);
        }
    }

    web::Json(OrderCalResult {
        price: order_total,
    })
}
