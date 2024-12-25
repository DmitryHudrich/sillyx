use clap::Arg;
use lazy_static::lazy_static;

lazy_static! {
    #[derive(Debug)]
    pub static ref CONFIG: Config = get_config();
}

#[derive(Debug)]
pub struct Config {
    pub salvo_addr: String,
    pub root_dir: String,
    pub cgi_ip: String,
    pub cgi_port: u16,
}

pub fn get_config() -> Config {
    let matches = clap::Command::new("sillyx")
        .version("1.0")
        .about("Sillyx - blazingly based Nginx alternative, written on rust.")
        .arg(
            Arg::new("php_fpm_port")
                .long("php-fpm-port")
                .default_value("9000")
                .help("The port of the PHP-FPM server"),
        )
        .arg(
            Arg::new("php_fpm_ip")
                .long("php-fpm-address")
                .default_value("127.0.0.1")
                .help("The address of the PHP-FPM server"),
        )
        .arg(
            Arg::new("root_dir")
                .long("root-dir")
                .default_value("/var/www/html")
                .help("The root directory for the PHP files"),
        )
        .arg(
            Arg::new("server_addr")
                .short('s')
                .long("server-addr")
                .default_value("127.0.0.1:1489")
                .help("Salvo listening address"),
        )
        .get_matches();

    Config {
        salvo_addr: matches
            .get_one::<String>("server_addr")
            .expect("Server addr required.")
            .clone(),
        root_dir: matches
            .get_one::<String>("root_dir")
            .expect("Root dir required.")
            .clone(),
        cgi_ip: matches
            .get_one::<String>("php_fpm_ip")
            .expect("Fpm ip required.")
            .clone(),
        cgi_port: matches
            .get_one::<String>("php_fpm_port")
            .expect("Fpm port required.")
            .parse::<u16>()
            .expect("Port must be an u16 number"),
    }
}
