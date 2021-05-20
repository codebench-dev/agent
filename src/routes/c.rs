use actix_web::{post, web, HttpResponse};

use crate::run::{self, c::compile_c};

use super::{RunCodeReq, RunRes};

pub enum Variant {
    GCC,
    Clang,
}

#[post("/run/c")]
pub async fn run_c(req: web::Json<RunCodeReq>) -> HttpResponse {
    let variant = match req.variant.as_str() {
        "gcc" => Variant::GCC,
        "clang" => Variant::Clang,
        _ => {
            return HttpResponse::BadRequest().json(RunRes {
                message: "Invalid language variant".to_string(),
                stdout: "".to_string(),
                stderr: "".to_string(),
            })
        }
    };

    let compile_res = compile_c(req.code.to_string(), variant);

    let binary_path = match compile_res {
        Err(err) => {
            return HttpResponse::InternalServerError().json(RunRes {
                message: err.to_string(),
                stdout: "".to_string(),
                stderr: "".to_string(),
            })
        }
        Ok(path) => path,
    };

    let exec_res = run::command::exec_binary(binary_path);

    match exec_res {
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
    async fn test_run_c_gcc() -> Result<(), Error> {
        let mut app = test::init_service(App::new().service(run_c)).await;

        let req = test::TestRequest::post()
            .uri("/run/c")
            .set_json(&RunCodeReq { id: "123".to_string(), code: "#include <stdio.h>\r\nint main() {\r\n   printf(\"Hello, C!\");\r\n   return 0;\r\n}".to_string(), variant: "gcc".to_string() })
            .to_request();
        let resp = app.call(req).await.unwrap();

        assert_eq!(resp.status(), http::StatusCode::OK);

        let response_body = match resp.response().body().as_ref() {
            Some(actix_web::body::Body::Bytes(bytes)) => bytes,
            _ => panic!("Response error"),
        };

        assert_eq!(
            response_body,
            r##"{"message":"","stdout":"Hello, C!","stderr":""}"##
        );

        Ok(())
    }

    #[actix_rt::test]
    async fn test_run_c_clang() -> Result<(), Error> {
        let mut app = test::init_service(App::new().service(run_c)).await;

        let req = test::TestRequest::post()
            .uri("/run/c")
            .set_json(&RunCodeReq { id: "123".to_string(), code: "#include <stdio.h>\r\nint main() {\r\n   printf(\"Hello, C!\");\r\n   return 0;\r\n}".to_string(), variant: "clang".to_string() })
            .to_request();
        let resp = app.call(req).await.unwrap();

        assert_eq!(resp.status(), http::StatusCode::OK);

        let response_body = match resp.response().body().as_ref() {
            Some(actix_web::body::Body::Bytes(bytes)) => bytes,
            _ => panic!("Response error"),
        };

        assert_eq!(
            response_body,
            r##"{"message":"","stdout":"Hello, C!","stderr":""}"##
        );

        Ok(())
    }
}
