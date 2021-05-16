use actix_web::{get, post, web, HttpResponse};
use serde::{Deserialize, Serialize};
use std::process::Command;

use crate::compile;
use crate::exec;

#[derive(Debug, Serialize, Deserialize)]
pub struct ExecReq {
    command: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExecRes {
    command: String,
    stdout: String,
    stderr: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RunCreq {
    id: String,
    code: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RunCres {
    message: String,
    stdout: String,
    stderr: String,
}

#[post("/run/c")]
pub async fn run_c(req: web::Json<RunCreq>) -> HttpResponse {
    let compile_res = compile::c::compile_c(req.code.to_string());

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

    let exec_res = exec::exec_binary(binary_path);

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

#[post("/exec")]
async fn exec_cmd(req: web::Json<ExecReq>) -> HttpResponse {
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
        let mut app = test::init_service(App::new().service(exec_cmd)).await;

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
