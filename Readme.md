# Sillyx

To use sillyx php-fpm must already be running, then:
- `cargo run`

Sillyx will be launched at `127.0.0.1:1489` and serve php files in `/var/www/html`

```text
Sillyx - blazingly based Nginx alternative, written on rust.

Usage: sillyx [OPTIONS]

Options:
      --php-fpm-port <php_fpm_port>   The port of the PHP-FPM server [default: 9000]
      --php-fpm-address <php_fpm_ip>  The address of the PHP-FPM server [default: 127.0.0.1]
      --root-dir <root_dir>           The root directory for the PHP files [default: /var/www/html]
  -s, --server-addr <server_addr>     Salvo listening address [default: 127.0.0.1:1489]
  -h, --help                          Print help
  -V, --version                       Print version
```
