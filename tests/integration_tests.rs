use actix_web::{test, App};
use crate::infrastructure::actix_web::configure_routes;

#[actix_rt::test]
async fn test_create_loan() {
    let app = test::init_service(App::new().configure(configure_routes)).await;
    let req = test::TestRequest::post()
       .uri("/loans")
       .set_json(&crate::domain::models::Loan {
            id: 1,
            applicant_name: "John Doe".to_string(),
            loan_amount: 1000,
            application_date: chrono::NaiveDate::from_ymd(2024, 1, 1),
            status: "Pending".to_string(),
        })
       .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
}