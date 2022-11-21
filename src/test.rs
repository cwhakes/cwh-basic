use super::rocket;

use rocket::http::Status;
use rocket::local::blocking::Client;

#[test]
fn health_check() {
    let client = Client::tracked(super::build()).expect("valid rocket instance");
    let response = client.get("/health_check").dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.into_string(), None);
}
