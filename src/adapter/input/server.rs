use std::sync::Arc;

use pgwire::{api::PgWireServerHandlers, tokio::process_socket};
use tokio::net::TcpListener;

pub async fn serve<A>(app: Arc<A>)
where
    A: PgWireServerHandlers + Send + Sync + 'static,
{
    let listner = TcpListener::bind("0.0.0.0:5432").await.unwrap();

    loop {
        let (socket, client) = listner.accept().await.unwrap();
        let app = app.clone();

        tokio::spawn(async move {
            if let Err(e) = process_socket(socket, None, app).await {
                eprintln!("Connection {:?} error: {:?}", client, e);
            };
        });
    }
}
