use std::collections::HashMap;

use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use actix_web_prom::PrometheusMetricsBuilder;

#[get("/health")]
async fn health() -> impl Responder {
    HttpResponse::Ok().finish()
}

#[get("/")]
async fn index() -> impl Responder {
    "Hello, World!"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut labels = HashMap::new();
    labels.insert("label1".to_string(), "value1".to_string());
    let prometheus = PrometheusMetricsBuilder::new("api")
        .endpoint("/metrics")
        .const_labels(labels)
        .build()
        .unwrap();

    HttpServer::new(move || {
        App::new()
            .wrap(prometheus.clone())
            .service(index)
            .service(health)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
