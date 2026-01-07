use aga8_api::api::create_api_service;
use poem::Server;
use poem::listener::TcpListener;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let api_service = create_api_service();
    let ui = api_service.swagger_ui();

    let app = poem::Route::new().nest("/", api_service).nest("/docs", ui);

    println!("Starting server at http://localhost:3000");
    println!("API documentation available at http://localhost:3000/docs");

    Server::new(TcpListener::bind("0.0.0.0:3000"))
        .run(app)
        .await
}
