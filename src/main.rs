use actix_web::{get, middleware, post, web, App, HttpResponse, HttpServer};
use serde::{Deserialize, Serialize};
use std::{
    fs, io,
    process::{Command, Output},
};

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

#[derive(Debug, Serialize, Deserialize)]
struct RunCreq {
    id: String,
    code: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct RunCres {
    message: String,
    stdout: String,
    stderr: String,
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
            .service(run_c)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}

#[post("/exec")]
async fn exec(req: web::Json<ExecReq>) -> HttpResponse {
    let cmd_args = req.command.split_whitespace().collect::<Vec<&str>>();
    let cmd_res = Command::new(cmd_args[0]).args(&cmd_args[1..]).output();

    match cmd_res {
        Err(_err) => HttpResponse::InternalServerError().finish(),
        Ok(output) => {
            let res = ExecRes {
                command: req.command.to_string(),
                stdout: String::from_utf8_lossy(&output.stdout).to_string(),
                stderr: String::from_utf8_lossy(&output.stderr).to_string(),
            };

            HttpResponse::Ok().json(res)
        }
    }
}

#[get("/health")]
async fn health() -> HttpResponse {
    HttpResponse::Ok().finish()
}

fn compile_c(req: RunCreq) -> Result<String, Box<dyn std::error::Error>> {
    fs::write(format!("/tmp/{}.c", req.id), req.code.clone())?;

    Command::new("gcc")
        .args(&[
            format!("/tmp/{}.c", req.id),
            "-o".to_string(),
            format!("/tmp/{}.out", req.id),
        ])
        .output()?;

    Ok(format!("/tmp/{}.out", req.id))
}

fn exec_binary(path: String) -> io::Result<Output> {
    Command::new(path).output()
}

#[post("/run/c")]
async fn run_c(req: web::Json<RunCreq>) -> HttpResponse {
    let compile_res = compile_c(RunCreq {
        id: req.id.clone(),
        code: req.code.clone(),
    });

    let binary_path = match compile_res {
        Err(err) => {
            return HttpResponse::InternalServerError().json(RunCres {
                message: err.to_string(),
                stdout: "".to_string(),
                stderr: "".to_string(),
            })
        }
        Ok(path) => path,
    };

    let exec_res = exec_binary(binary_path);

    match exec_res {
        Err(_err) => HttpResponse::InternalServerError().finish(),
        Ok(output) => {
            let res = RunCres {
                message: "stonks".to_string(),
                stdout: String::from_utf8_lossy(&output.stdout).to_string(),
                stderr: String::from_utf8_lossy(&output.stderr).to_string(),
            };

            HttpResponse::Ok().json(res)
        }
    }
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

    #[actix_rt::test]
    async fn test_run_c() -> Result<(), Error> {
        let mut app = test::init_service(App::new().service(run_c)).await;

        let req = test::TestRequest::post()
            .uri("/run/c")
            .set_json(&RunCreq { id: "123".to_string(), code: "#include <stdio.h>\r\nint main() {\r\n   printf(\"Hello, World!\");\r\n   return 0;\r\n}".to_string()})
            .to_request();
        let resp = app.call(req).await.unwrap();

        assert_eq!(resp.status(), http::StatusCode::OK);

        let response_body = match resp.response().body().as_ref() {
            Some(actix_web::body::Body::Bytes(bytes)) => bytes,
            _ => panic!("Response error"),
        };

        assert_eq!(
            response_body,
            r##"{"message":"stonks","stdout":"Hello, World!","stderr":""}"##
        );

        Ok(())
    }
}
