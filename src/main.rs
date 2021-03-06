mod models;
mod vader;
use actix_web::{middleware, post, web, App, HttpResponse, HttpServer, Responder};
use models::RequestText;
use models::VaderScores;
use vader::SentimentIntensityAnalyzer;

#[post("/get_sentiment")]
async fn get_sentiment(req_text: web::Json<RequestText>) -> impl Responder {
    let analyzer = SentimentIntensityAnalyzer::new();
    let response: Vec<VaderScores> = req_text
        .text
        .iter()
        .map(|text| analyzer.polarity_scores(text))
        .collect();
    HttpResponse::Ok().json(response)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    println!("starting server at http://0.0.0.0:8080/");

    HttpServer::new(|| {
        App::new()
            // overide default 256kb payload limit with arbitrarily high value
            .data(web::JsonConfig::default().limit(9096000))
            .wrap(middleware::Logger::new(
                "%t %r %s %b %{Referer}i %{User-Agent}i %T",
            ))
            .service(get_sentiment)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
