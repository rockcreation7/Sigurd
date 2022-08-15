use actix_web::{web, post, App, HttpServer, Responder}; 
use awc::Client; // https://crates.io/crates/awc //https://github.com/actix/examples/blob/master/https-tls/awc-https/src/main.rs
use serde::{Deserialize, Serialize};
mod key;
/* use crate::game::*; */
#[actix_web::main] 
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(calculate_ticket)
        
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

#[derive(Debug, Serialize, Deserialize)]
struct TicketCalResult {
    price: f32,
    is_toy: bool,
    ticket: f32,
    ticketdiscount: f32,
}

#[derive(Debug, Deserialize, Clone)]
struct TicketCalData {
    price: f32,
    is_toy: bool,
    order: Vec<Item>,
}

#[derive(Debug, Deserialize, Clone)]
struct Item {
    id: f32,
    qty: i32,
}

#[post("/calculate_order")]
async fn calculate_ticket(data: web::Json<TicketCalData>) -> impl Responder {
    // make request here
    let client = Client::default();

    let res = client
        .get("http://www.rust-lang.org")    // <- Create request builder
        .insert_header(("User-Agent", "Actix-web"))
        .send()                             // <- Send http request
        .await;

    println!("Response: {:?}", res);     
    // make request here
  
    println!("model: {:?}", &data);
    println!("{} {}", key::BOT_KEY, key::CHAT_ID);
    let ticket: f32;

    let ticketforprice: f32 = data.price / 10.0;
    let ticketforplay: f32 = ticketforprice * 5.0 / 10.0;
    ticket = ticketforprice + ticketforplay;

    for elem in data.order.clone() {
        println!("{} {}", elem.id, elem.qty);
    }

    println!("{:#?}", ticket);

    let mut ticketdiscount: f32 = 0.0;

    if data.is_toy && ((data.price / 100.0) > 1.0) {
        ticketdiscount = (data.price / 100.0).floor()
    }

    web::Json(TicketCalResult {
        price: 0.0,
        is_toy: true,
        ticket: ticket.round(),
        ticketdiscount: ticketdiscount,
    })
}