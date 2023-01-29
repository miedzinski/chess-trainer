use std::net::SocketAddr;

use chess_trainer::infrastructure::rest::make_router;
use chess_trainer::puzzle;

#[tokio::main]
async fn main() {
    let puzzle_service = puzzle::make_service();
    let app = make_router(puzzle_service);

    let addr = SocketAddr::from(([0, 0, 0, 0], 5000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
