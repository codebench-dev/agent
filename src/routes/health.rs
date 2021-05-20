use actix_web::{get, HttpResponse};

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
}
