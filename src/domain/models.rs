use diesel::prelude::*;

#[derive(Queryable, Insertable, Serialize, Deserialize)]
#[table_name = "loans"]
pub struct Loan {
    pub id: i32,
    pub applicant_name: String,
    pub loan_amount: i32,
    pub application_date: NaiveDate,
    pub status: String,
}