use config::CONFIG;
use salvo::{http::HeaderMap, prelude::*};
use tracing::info;

mod cgi;
mod config;
mod handler_tools;

#[handler]
async fn handle_endpoint(req: &Request, res: &mut Response) {
    let endpoint = req.uri().path().to_string();
    let cgi_output = cgi::cgi_request(endpoint).await;

    let (headers, body) = handler_tools::sep_headers(cgi_output);

    res.set_headers(HeaderMap::from_iter(handler_tools::parse_headers(headers)));
    res.headers_mut().insert("server", "sillyx".parse().unwrap());
    res.render(Text::Html(body));
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let router = Router::new()
        .push(Router::with_path("<path>").get(handle_endpoint))
        .get(handle_endpoint);

    let acceptor = TcpListener::new(&CONFIG.salvo_addr).bind().await;
    Server::new(acceptor).serve(router).await;

    info!("listen in {}", &CONFIG.salvo_addr);
}
