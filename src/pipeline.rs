use std::collections::HashMap;

use salvo::{
    http::HeaderMap,
    writing::{Redirect, Text},
    Request, Response,
};
use tokio::{fs::File, io::AsyncReadExt};
use tracing::*;

use crate::{cgi, config::CONFIG, handler_tools};

pub(crate) struct PipelineContainer<'a> {
    response: &'a mut Response,
    request: &'a mut Request,
    endpoint: String,
    query_string: String,
}

impl<'a> PipelineContainer<'a> {
    pub(crate) fn new(response: &'a mut Response, request: &'a mut Request) -> Self {
        let uri = &request.uri();
        Self {
            response,
            endpoint: uri.path().to_string().to_owned(),
            query_string: handler_tools::extract_query_string(request),
            request,
        }
    }

    pub(crate) async fn dispatch_endpoint_type(&mut self) {
        match self.endpoint {
            _ if !self.endpoint.contains(".") => {
                debug!(
                    "Redirecting: {}/ -> {}/index.php",
                    self.endpoint, self.endpoint
                );
                self.endpoint += "index.php";
                self.process_php().await
            }
            _ if self.endpoint.ends_with(".php") => {
                debug!("Sending to php-fpm: {}", self.endpoint);
                self.process_php().await
            }
            _ => {
                debug!("Sending static: {}", self.endpoint);
                self.try_process_static().await
            }
        };
        self.redirect_if_needed();
    }

    fn redirect_if_needed(&mut self) {
        if self.response.headers().contains_key("location") {
            self.response.render(Redirect::permanent(
                self.response.headers()["location"].to_str().unwrap(),
            ))
        }
    }

    async fn try_process_static(&mut self) {
        let path_to_content = format!("{}{}", CONFIG.root_dir, self.endpoint).replace("//", "/");
        let file = File::open(&path_to_content).await;
        let mut content = String::new();
        if file.is_ok() && file.unwrap().read_to_string(&mut content).await.is_ok() {
            self.process_static(content, path_to_content).await;
        } else {
            debug!("File {} not found.", path_to_content);
        }
    }

    async fn process_static(&mut self, content: String, path_to_content: String) {
        match self.endpoint {
            _ if self.endpoint.ends_with(".css") => {
                debug!("Rendering CSS content for endpoint: {}", self.endpoint);
                self.response.render(Text::Css(content));
            }
            _ if self.endpoint.ends_with(".js") => {
                debug!("Rendering JS content for endpoint: {}", self.endpoint);
                self.response.render(Text::Js(content));
            }
            _ if self.endpoint.ends_with(".html") || self.endpoint.ends_with("htm") => {
                debug!("Rendering HTML content for endpoint: {}", self.endpoint);
                self.response.render(Text::Html(content));
            }
            _ => {
                debug!("Sending file for endpoint: {}", self.endpoint);
                self.response
                    .send_file(path_to_content, self.request.headers())
                    .await;
            }
        }
    }

    async fn process_php(&mut self) {
        let cgi_output = cgi::cgi_request(&self.endpoint, &self.query_string).await;
        let (headers, body) = handler_tools::sep_headers(cgi_output);
        debug!("Processing PHP: {}", self.endpoint);
        let parsed_headers = handler_tools::parse_headers(headers);
        self.response
            .set_headers(HeaderMap::from_iter(parsed_headers.clone()));
        self.response
            .headers_mut()
            .insert("server", "sillyx".parse().unwrap());
        self.check_need_redirect(parsed_headers);
        self.response.render(Text::Html(body));
    }

    fn check_need_redirect(
        &mut self,
        parsed_headers: HashMap<salvo::http::HeaderName, salvo::http::HeaderValue>,
    ) -> bool {
        for (key, value) in &parsed_headers {
            if key.to_string().contains("location") {
                debug!("Redirect to {}", value.to_str().unwrap());
                self.response
                    .headers_mut()
                    .insert("location", value.to_str().unwrap().parse().unwrap());
                return true;
            }
        }
        false
    }
}
