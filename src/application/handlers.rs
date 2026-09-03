use actix_web::{web, HttpResponse, Responder};
use diesel::prelude::*;
use crate::domain::models::Loan;
use crate::infrastructure::database::establish_connection;

pub async fn create_loan(web::Json(new_loan): web::Json<Loan>) -> impl Responder {
    let connection = &mut establish_connection();
    diesel::insert_into(loans::table)
       .values(&new_loan)
       .execute(connection)
       .expect("Error saving new loan");
    HttpResponse::Ok().json(new_loan)
}