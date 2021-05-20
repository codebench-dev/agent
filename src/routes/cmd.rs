use std::process::Command;

use actix_web::{post, web, HttpResponse};
use serde::{Deserialize, Serialize};

use super::RunRes;

#[derive(Debug, Serialize, Deserialize)]
pub struct ExecCmdReq {
    command: String,
}

#[post("/run/cmd")]
async fn run_cmd(req: web::Json<ExecCmdReq>) -> HttpResponse {
    let cmd_args = req.command.split_whitespace().collect::<Vec<&str>>();
    let cmd_res = Command::new(cmd_args[0]).args(&cmd_args[1..]).output();

    match cmd_res {
        Err(_err) => HttpResponse::InternalServerError().finish(),
        Ok(output) => {
            let res = RunRes {
                message: "".to_string(),
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
    async fn test_exec() -> Result<(), Error> {
        let mut app = test::init_service(App::new().service(run_cmd)).await;

        let req = test::TestRequest::post()
            .uri("/run/cmd")
            .set_json(&ExecCmdReq {
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
            r##"{"message":"","stdout":"Linux\n","stderr":""}"##
        );

        Ok(())
    }
}
