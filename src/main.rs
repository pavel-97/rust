pub mod adapter;

use std::sync::Arc;

use pgwire::api::PgWireServerHandlers;
use pgwire::tokio::process_socket;
use tokio::net::{TcpListener, TcpStream};

use crate::adapter::input::server::serve;

struct Factory;

impl PgWireServerHandlers for Factory {}

#[tokio::main]
async fn main() {
    let factory = Arc::new(Factory);

    serve(factory).await;
}
