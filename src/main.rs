use actix_web::{middleware, web, App, HttpServer};

mod routes;
mod run;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            // enable logger
            .wrap(middleware::Logger::default())
            .data(web::JsonConfig::default().limit(4096)) // <- limit size of the payload (global configuration)
            .service(routes::health::health)
            .service(routes::cmd::run_cmd)
            .service(routes::c::run_c)
            .service(routes::python::run_python)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
