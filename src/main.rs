use actix_web::{middleware, web, App, HttpResponse, HttpServer};
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

async fn exec(req: web::Json<ExecReq>) -> HttpResponse {
    println!("model: {:?}", &req);

    let cmd_args = req.command.split_whitespace().collect::<Vec<&str>>();
    let cmd_exec = cmd_args[0];
    let cmd_res = Command::new(cmd_exec).args(&cmd_args[1..]).output();

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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            // enable logger
            .wrap(middleware::Logger::default())
            .data(web::JsonConfig::default().limit(4096)) // <- limit size of the payload (global configuration)
            .service(web::resource("/exec").route(web::post().to(exec)))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use actix_web::dev::Service;
//     use actix_web::{http, test, web, App};

//     #[actix_rt::test]
//     async fn test_index() -> Result<(), Error> {
//         let mut app =
//             test::init_service(App::new().service(web::resource("/").route(web::post().to(index))))
//                 .await;

//         let req = test::TestRequest::post()
//             .uri("/")
//             .set_json(&MyObj {
//                 name: "my-name".to_owned(),
//                 number: 43,
//             })
//             .to_request();
//         let resp = app.call(req).await.unwrap();

//         assert_eq!(resp.status(), http::StatusCode::OK);

//         let response_body = match resp.response().body().as_ref() {
//             Some(actix_web::body::Body::Bytes(bytes)) => bytes,
//             _ => panic!("Response error"),
//         };

//         assert_eq!(response_body, r##"{"name":"my-name","number":43}"##);

//         Ok(())
//     }
// }
