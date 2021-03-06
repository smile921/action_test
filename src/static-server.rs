use actix_files as fs; 
use std::fs::File;
use std::io::BufReader;

use actix_files::Files;
use actix_web::{middleware,web, App, HttpRequest, HttpResponse, HttpServer};
use rustls::internal::pemfile::{certs, pkcs8_private_keys};
use rustls::{NoClientAuth, ServerConfig};

async fn index(req: HttpRequest) -> HttpResponse {
    println!("{:?}", req);
    HttpResponse::Ok()
        .content_type("text/plain")
        .body("Welcome!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }
    env_logger::init();
    // load ssl keys
    let mut config = ServerConfig::new(NoClientAuth::new());
     
    let cert_file = &mut BufReader::new(File::open("/Users/macbook/work_money/smile921/dist/cert.pem").unwrap());
    let key_file = &mut BufReader::new(File::open("/Users/macbook/work_money/smile921/dist/key.pem").unwrap());
    let cert_chain = certs(cert_file).unwrap();
    let mut keys = pkcs8_private_keys(key_file).unwrap();
    config.set_single_cert(cert_chain, keys.remove(0)).unwrap();
    
    println!("static server starting ... at https://localhost/static/index.html ");
    println!("static server starting ... at https://smile921.org/static/index.html ");
    HttpServer::new(|| {
        App::new()
            // enable logger
            .wrap(middleware::Logger::default())
            // register simple handler, handle all methods
             // register simple handler, handle all methods
             .service(web::resource("/index.html").to(index))
             // with path parameters
             .service(web::resource("/").route(web::get().to(|| {
                 HttpResponse::Found()
                     .header("LOCATION", "/index.html")
                     .finish()
             })))
            // .service(fs::Files::new("/static", ".").show_files_listing())            
            .service(Files::new("/static", "/Users/macbook/work_money/smile921/dist/static"))
    })
    .bind_rustls("127.0.0.1:443", config)?
    .run()
    .await
    
} 