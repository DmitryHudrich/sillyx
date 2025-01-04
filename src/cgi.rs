use std::borrow::Cow;

use tokio::io;

use crate::config::CONFIG;

pub async fn cgi_request(endpoint: impl AsRef<str>, query: impl AsRef<str>) -> String {
    let script_name = endpoint.as_ref().to_string();

    let stream = tokio::net::TcpStream::connect((CONFIG.cgi_ip.as_ref(), CONFIG.cgi_port))
        .await
        .unwrap();

    let client = fastcgi_client::Client::new(stream);
    let file = endpoint.as_ref().to_string();
    let mut script_filename = format!("{}{}", CONFIG.root_dir, file);
    script_filename = script_filename.replace("//", "/");

    let params = set_cgi_params(query.as_ref(), script_name, script_filename);

    let output = client
        .execute_once(fastcgi_client::Request::new(params, &mut io::empty()))
        .await
        .unwrap();

    String::from_utf8(output.stdout.unwrap()).unwrap()
}

fn set_cgi_params<'a>(
    query: impl Into<Cow<'a, str>>,
    script_name: impl Into<Cow<'a, str>> + Clone,
    script_filename: impl Into<Cow<'a, str>>,
) -> fastcgi_client::Params<'a> {
    let mut params = fastcgi_client::Params::default()
        // тут большинство из возможных переменных, которые ставит nginx
        // --------------------------------------------------------
        // .insert_custom("HTTP_HOST", "127.0.0.1")
        // .insert_custom("USER", "eblan")
        // .insert_custom("HOME", "/srv/http")
        // .insert_custom("HTTP_PRIORITY", "u=0, i")
        // .insert_custom("HTTP_SEC_FETCH_USER", "?1")
        // .insert_custom("HTTP_SEC_FETCH_SITE", "same-origin")
        // .insert_custom("HTTP_SEC_FETCH_MODE", "navigate")
        // .insert_custom("HTTP_SEC_FETCH_DEST", "document")
        // .insert_custom("HTTP_UPGRADE_INSECURE_REQUESTS", "1")
        // .insert_custom("HTTP_COOKIE", "io=c4XMd-DGBX1eiH0LAAAA")
        // .insert_custom("HTTP_CONNECTION", "keep-alive")
        // // .insert_custom("HTTP_REFERER", "http://localhost:5321/index.php")
        // .insert_custom("HTTP_ACCEPT_ENCODING", "gzip, deflate, br, zstd")
        // .insert_custom("HTTP_ACCEPT_LANGUAGE", "en-US,en;q=0.5")
        // .insert_custom(
        //     "HTTP_ACCEPT",
        //     "text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8",
        // )
        // .insert_custom(
        //     "HTTP_USER_AGENT",
        //     "Mozilla/5.0 (X11; Linux x86_64; rv:133.0) Gecko/20100101 Firefox/133.0",
        // )
        // .insert_custom("FCGI_ROLE", "RESPONDER")
        // .insert_custom("PHP_SELF", "") // no value
        // .insert_custom("REQUEST_TIME_FLOAT", "1735893922.0193")
        // .insert_custom("REQUEST_TIME", "1735893922")
        .query_string(query)
        .request_method("GET") // not important yet
        .script_name(script_name.clone())
        .script_filename(script_filename)
        .request_uri(script_name.clone())
        // it's not important, i think
        .remote_addr("127.0.0.1")
        .remote_port(23421)
        .server_addr("127.0.0.1")
        .server_port(
            CONFIG
                .salvo_addr
                .split(":")
                .nth(1)
                .unwrap()
                .parse()
                .unwrap(),
        )
        .server_name("zaza")
        .content_type("")
        .content_length(0)
        .document_uri(script_name);

    params.insert("HTTP_HOST".into(), (&CONFIG.salvo_addr).into());
    params
}
