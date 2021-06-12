use actix_web::{post, web, HttpResponse};
use chrono::Utc;

use crate::run::{self, go::compile_go};

use super::{RunCodeReq, RunRes};

pub enum Variant {
    Vanilla,
}

#[post("/run/go")]
pub async fn run_go(req: web::Json<RunCodeReq>) -> HttpResponse {
    let variant = match req.variant.as_str() {
        "vanilla" => Variant::Vanilla,
        _ => {
            return HttpResponse::BadRequest().json(RunRes {
                message: "Invalid language variant".to_string(),
                stdout: "".to_string(),
                stderr: "".to_string(),
                exec_duration: 0,
            })
        }
    };

    let compile_res = compile_go(req.code.to_string(), variant);

    let binary_path = match compile_res {
        Err(err) => {
            return HttpResponse::BadRequest().json(RunRes {
                message: "Failed to compile".to_string(),
                stdout: "".to_string(),
                stderr: err.to_string(),
                exec_duration: 0,
            })
        }
        Ok(path) => path,
    };

    let start_time = Utc::now().time();
    let exec_res = run::command::exec_binary(binary_path);
    let end_time = Utc::now().time();
    let diff = end_time - start_time;

    match exec_res {
        Err(err) => {
            return HttpResponse::BadRequest().json(RunRes {
                message: "Failed to execute code".to_string(),
                stdout: "".to_string(),
                stderr: err.to_string(),
                exec_duration: diff.num_milliseconds().abs(),
            })
        }
        Ok(output) => {
            let res = RunRes {
                message: "OK".to_string(),
                stdout: String::from_utf8_lossy(&output.stdout).to_string(),
                stderr: String::from_utf8_lossy(&output.stderr).to_string(),
                exec_duration: diff.num_milliseconds().abs(),
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
    use pretty_assertions::assert_eq;

    #[actix_rt::test]
    async fn test_run_go() -> Result<(), Error> {
        let mut app = test::init_service(App::new().service(run_go)).await;

        let req = test::TestRequest::post()
            .uri("/run/go")
            .set_json(&RunCodeReq {
                id: "123".to_string(),
                code: "package main\r\n\r\nimport \"fmt\"\r\n\r\nfunc main() {\r\n    fmt.Println(\"Hello, Go!\")\r\n}".to_string(),
                variant: "vanilla".to_string(),
            })
            .to_request();
        let resp = app.call(req).await.unwrap();

        assert_eq!(resp.status(), http::StatusCode::OK);

        // let response_body = match resp.response().body().as_ref() {
        //     Some(actix_web::body::Body::Bytes(bytes)) => bytes,
        //     _ => panic!("Response error"),
        // };

        // assert_eq!(
        //     response_body,
        //     r##"{"message":"OK","stdout":"Hello, Go!\n","stderr":""}"##
        // );

        Ok(())
    }
}
