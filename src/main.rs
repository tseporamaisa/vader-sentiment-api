mod models;
mod vader;
//use crate::models::RequestText;
use crate::models::VaderScores;
use actix_web::{post, web, App, HttpResponse, HttpServer, Responder};
use vader::SentimentIntensityAnalyzer;

#[post("/get_sentiment")]
async fn get_sentiment(req_text: web::Json<Vec<String>>) -> impl Responder {
    let analyzer = SentimentIntensityAnalyzer::new();
    let response: Vec<VaderScores> = req_text
        .iter()
        .map(|text| analyzer.polarity_scores(text))
        .collect();
    HttpResponse::Ok().json(response)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("starting server at http://0.0.0.0:8080/");

    HttpServer::new(|| App::new().service(get_sentiment))
        .bind("0.0.0.0:8080")?
        .run()
        .await
}
