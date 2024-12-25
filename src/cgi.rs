use tokio::io;

use crate::config::CONFIG;

pub async fn cgi_request(endpoint: impl AsRef<str>) -> String {
    let script_name = endpoint.as_ref().to_string();

    // Connect to php-fpm default listening address.
    let stream = tokio::net::TcpStream::connect((CONFIG.cgi_ip.as_ref(), CONFIG.cgi_port))
        .await
        .unwrap();
    let client = fastcgi_client::Client::new(stream);

    let params = fastcgi_client::Params::default()
        .request_method("GET")
        .script_name(&script_name)
        .script_filename(format!("{}{}", CONFIG.root_dir, endpoint.as_ref()))
        .request_uri(&script_name)
        .document_uri(&script_name);

    // it's not important, i think
    // .remote_addr("127.0.0.1")
    // .remote_port(23421)
    // .server_addr("127.0.0.1")
    // .server_port(80)
    // .server_name("zaza")
    // .content_type("")
    // .content_length(0);

    let output = client
        .execute_once(fastcgi_client::Request::new(params, &mut io::empty()))
        .await
        .unwrap();

    // "Content-type: text/html; charset=UTF-8\r\n\r\nhello"
    String::from_utf8(output.stdout.unwrap()).unwrap()
}
