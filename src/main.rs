use actix_web::{middleware, web, App, HttpServer};

mod compile;
mod exec;
mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            // enable logger
            .wrap(middleware::Logger::default())
            .data(web::JsonConfig::default().limit(4096)) // <- limit size of the payload (global configuration)
            .service(routes::exec_cmd)
            .service(routes::health)
            .service(routes::run_c)
            .service(routes::run_python2)
            .service(routes::run_python3)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
