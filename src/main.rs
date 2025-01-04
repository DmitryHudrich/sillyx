use config::CONFIG;
use pipeline::PipelineContainer;
use salvo::prelude::*;
use tracing::{info, Level};

mod cgi;
mod config;
mod handler_tools;

mod pipeline;

#[handler]
async fn handle_endpoint(req: &mut Request, res: &mut Response) {
    let mut pipeliner = PipelineContainer::new(res, req);
    pipeliner.dispatch_endpoint_type().await;
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .init();
    info!("~>_<~");

    let router = Router::new()
        .push(
            Router::with_path("<**path>")
                .get(handle_endpoint)
                .post(handle_endpoint),
        )
        .get(handle_endpoint);

    let acceptor = TcpListener::new(&CONFIG.salvo_addr).bind().await;
    Server::new(acceptor).serve(router).await;

    info!("Listen in {}", &CONFIG.salvo_addr);
}
