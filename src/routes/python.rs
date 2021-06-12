use std::fs;

use actix_web::{post, web, HttpResponse};
use chrono::Utc;

use crate::run::python;

use super::{RunCodeReq, RunRes};

pub enum Variant {
    CPython2,
    CPython3,
}

#[post("/run/python")]
pub async fn run_python(req: web::Json<RunCodeReq>) -> HttpResponse {
    let variant = match req.variant.as_str() {
        "cpython2" => Variant::CPython2,
        "cpython3" => Variant::CPython3,
        _ => {
            return HttpResponse::BadRequest().json(RunRes {
                message: "Invalid language variant".to_string(),
                stdout: "".to_string(),
                stderr: "".to_string(),
                exec_duration: 0,
            })
        }
    };

    let filename = format!("/tmp/{}.python", req.id);

    if let Err(err) = fs::write(filename.clone(), req.code.clone()) {
        return HttpResponse::InternalServerError().json(RunRes {
            message: "Failed to write code to file".to_string(),
            stdout: "".to_string(),
            stderr: err.to_string(),
            exec_duration: 0,
        });
    }

    let start_time = Utc::now().time();
    let exec_res = python::run_python(filename.clone(), variant);
    let end_time = Utc::now().time();
    let diff = end_time - start_time;

    match exec_res {
        Err(err) => {
            return HttpResponse::BadRequest().json(RunRes {
                message: "Failed to run".to_string(),
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
    use uuid::Uuid;

    #[actix_rt::test]
    async fn test_run_python2() -> Result<(), Error> {
        let mut app = test::init_service(App::new().service(run_python)).await;

        let req = test::TestRequest::post()
            .uri("/run/python")
            .set_json(&RunCodeReq {
                id: Uuid::new_v4().to_string(),
                code: "print 'Hello, Python 2!'".to_string(),
                variant: "cpython2".to_string(),
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
        //     r##"{"message":"OK","stdout":"Hello, Python 2!\n","stderr":""}"##
        // );

        Ok(())
    }

    #[actix_rt::test]
    async fn test_run_python3() -> Result<(), Error> {
        let mut app = test::init_service(App::new().service(run_python)).await;

        let req = test::TestRequest::post()
            .uri("/run/python")
            .set_json(&RunCodeReq {
                id: Uuid::new_v4().to_string(),
                code: "print('Hello, Python 3!')".to_string(),
                variant: "cpython3".to_string(),
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
        //     r##"{"message":"OK","stdout":"Hello, Python 3!\n","stderr":""}"##
        // );

        Ok(())
    }
}
