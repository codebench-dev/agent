use actix_web::{get, middleware, post, web, App, HttpResponse, HttpServer};
use serde::{Deserialize, Serialize};
use std::process::Command;

#[derive(Debug, Serialize, Deserialize)]
struct ExecReq {
    command: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct ExecRes {
    command: String,
    stdout: String,
    stderr: String,
}

#[post("/exec")]
async fn exec(req: web::Json<ExecReq>) -> HttpResponse {
    let cmd_args = req.command.split_whitespace().collect::<Vec<&str>>();
    let cmd_res = Command::new(cmd_args[0]).args(&cmd_args[1..]).output();

    match cmd_res {
        Ok(output) => {
            let res = ExecRes {
                command: req.command.to_string(),
                stdout: String::from_utf8_lossy(&output.stdout).to_string(),
                stderr: String::from_utf8_lossy(&output.stderr).to_string(),
            };

            HttpResponse::Ok().json(res)
        }
        Err(err) => HttpResponse::InternalServerError().json(err.to_string()),
    }
}

#[get("/health")]
async fn health() -> HttpResponse {
    HttpResponse::Ok().finish()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            // enable logger
            .wrap(middleware::Logger::default())
            .data(web::JsonConfig::default().limit(4096)) // <- limit size of the payload (global configuration)
            .service(exec)
            .service(health)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{dev::Service, Error};
    use actix_web::{http, test, App};

    #[actix_rt::test]
    async fn test_health() -> Result<(), Error> {
        let mut app = test::init_service(App::new().service(health)).await;

        let req = test::TestRequest::get().uri("/health").to_request();
        let resp = app.call(req).await.unwrap();

        assert_eq!(resp.status(), http::StatusCode::OK);

        Ok(())
    }

    #[actix_rt::test]
    async fn test_exec() -> Result<(), Error> {
        let mut app = test::init_service(App::new().service(exec)).await;

        let req = test::TestRequest::post()
            .uri("/exec")
            .set_json(&ExecReq {
                command: "uname".to_string(),
            })
            .to_request();
        let resp = app.call(req).await.unwrap();

        assert_eq!(resp.status(), http::StatusCode::OK);

        let response_body = match resp.response().body().as_ref() {
            Some(actix_web::body::Body::Bytes(bytes)) => bytes,
            _ => panic!("Response error"),
        };

        assert_eq!(
            response_body,
            r##"{"command":"uname","stdout":"Linux\n","stderr":""}"##
        );

        Ok(())
    }
}
