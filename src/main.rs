use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use actix_cors::Cors;

#[get("/video")]
async fn video() -> impl Responder {
    let video_url = "https://www.w3schools.com/html/mov_bbb.mp4"; // Example video URL
    let html = format!(
        r#"
        <!DOCTYPE html>
        <html lang="en">
        <head>
            <meta charset="UTF-8">
            <meta name="viewport" content="width=device-width, initial-scale=1.0">
            <title>Video Player</title>
        </head>
        <body>
            <h1>Video Player</h1>
            <video width="320" height="240" controls>
                <source src="{}" type="video/mp4">
                Your browser does not support the video tag.
            </video>
        </body>
        </html>
        "#,
        video_url
    );

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(Cors::permissive()) // Allow all CORS requests for simplicity
            .service(video)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
