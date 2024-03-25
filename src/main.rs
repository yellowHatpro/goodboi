mod app;
mod remember;
mod structs;
mod utils;

#[tokio::main]
async fn main() {
    app::start().await;
}