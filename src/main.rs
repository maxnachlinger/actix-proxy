use crate::settings::{AllowList, Settings};
use actix_web::client::Client;
use actix_web::{middleware, web, App, Error, HttpMessage, HttpRequest, HttpResponse, HttpServer};
use std::net::ToSocketAddrs;
use url::Url;
mod settings;

async fn forward(
    req: HttpRequest,
    body: web::Bytes,
    url: web::Data<Url>,
    client: web::Data<Client>,
    allowlist: web::Data<AllowList>,
) -> Result<HttpResponse, Error> {
    let mut new_url = url.get_ref().clone();
    new_url.set_path(req.uri().path());
    new_url.set_query(req.uri().query());

    let mut forwarded_req = client
        .request(req.method().clone(), new_url.as_str())
        .no_decompress();

    // add allowed headers
    for (header_name, header_value) in req
        .headers()
        .iter()
        .filter(|(header_name, _)| allowlist.header_names.contains(&header_name.to_string()))
    {
        forwarded_req = forwarded_req.header(header_name.clone(), header_value.clone());
    }

    // add allowed cookies
    for cookie in req
        .cookies()?
        .iter()
        .filter(|c| allowlist.cookie_names.contains(&String::from(c.name())))
    {
        forwarded_req = forwarded_req.cookie(cookie.clone());
    }

    let mut res = forwarded_req.send_body(body).await.map_err(Error::from)?;

    let mut client_resp = HttpResponse::build(res.status());

    // Remove `Connection` as per
    // https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Connection#Directives
    for (header_name, header_value) in res.headers().iter().filter(|(h, _)| *h != "connection") {
        client_resp.header(header_name.clone(), header_value.clone());
    }

    Ok(client_resp.body(res.body().await?))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let settings = Settings::new().expect("Could not read settings");
    let listen_address = settings.app.clone().listen_address;

    let forward_url = Url::parse(&format!(
        "http://{}",
        (&settings.app.forward_address)
            .to_socket_addrs()
            .unwrap()
            .next()
            .unwrap()
    ))
    .unwrap();

    HttpServer::new(move || {
        App::new()
            .data(Client::new())
            .data(forward_url.clone())
            .data(settings.allowlist.clone())
            .wrap(middleware::Logger::default())
            .default_service(web::route().to(forward))
    })
    .bind(listen_address)?
    .system_exit()
    .run()
    .await
}
