use actix_web::{get, post, HttpResponse, Responder, web};

// This struct represents state
pub(crate) struct AppState {
    pub(crate) app_name: String,
}

#[get("/")]
pub(crate) async fn index(data: web::Data<AppState>) -> String {
    let app_name = &data.app_name; // <- get app_name
    format!("Hello {app_name}!") // <- response with app_name
}

#[post("/echo")]
pub(crate) async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

pub(crate) async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}