mod yellowpagescrapper;
mod facebookcheck;
mod googlemap;

use actix_web::{self, HttpServer, App, Responder, get, HttpResponse};
use googlemap::google_map_scrapper;



#[actix_web::main]
async fn main() {
    //start server
    //scrap_yellowpage().await;
    //let _a = facebook_check().await;
    //let _b = google_map_scrapper().await;
    
    println!("actix web go!");
    HttpServer::new(|| {
        App::new()
        .service(run_googlemap)
        .service(stop)
    })
    .bind(("127.0.0.1", 8080))
    .unwrap()
    .workers(4)
    .run()
    .await
    .unwrap();
    

    
  
}
/* 
#[get("/runlppeh")]
async fn run_lppeh() -> impl Responder {
    let result = scrap_lppeh().await;
    
    if result == true {
        return HttpResponse::Ok()
    } else {
        return HttpResponse::Ok()
    }
    
}
*/

/* 
#[get("runyellowpage")]
async fn run_yellowpage() -> impl Responder {
    scrap_yellowpage().await;
    HttpResponse::Ok()
}
*/

#[get("/stop")]
async fn stop() -> impl Responder {
    HttpResponse::Ok()
}

#[get("/googlemap")]
async fn run_googlemap() -> impl Responder {
    google_map_scrapper().await;
    HttpResponse::Ok()
}


