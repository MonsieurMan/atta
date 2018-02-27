use flate2::read::GzDecoder;
use tar::Archive;

use std::io;

use serde_json;
use serde_json::Value;

use futures::{Future, Stream};
use futures::stream::Concat2;

use hyper;
use hyper::{Body, Chunk, Client, Response};
use hyper::client::HttpConnector;

pub fn download_package(
    name: &'static str,
    client: Client<HttpConnector>,
) -> Box<Future<Item = Result<(), io::Error>, Error = hyper::Error>> {
    let work = download_package_json(name, client.clone())
        .map(retrieve_tarball_link)
        .and_then(move |link| download_tarbal(&link[..], client.clone()))
        .map(move |chunk| write_package(name, chunk));
    Box::new(work)
}

fn retrieve_tarball_link(package: Value) -> String {
    let link = &package["dist"]["tarball"].to_string();
    (&link[1..link.len() - 1]).to_string()
}

fn write_package(name: &str, bytes: Chunk) -> Result<(), io::Error> {
    let d = GzDecoder::new(bytes.as_ref());
    let mut a = Archive::new(d);
    let path = format!("node_modules/{}", name);
    println!("Writing to {}", path);
    a.unpack(path)
}

fn download_tarbal(
    uri: &str,
    client: Client<HttpConnector>,
) -> Box<Future<Item = Chunk, Error = hyper::Error>> {
    println!("Downloading {}", uri);
    let tarball = client.get(uri.parse().unwrap()).and_then(concat_body);
    Box::new(tarball)
}

fn download_package_json(
    name: &str,
    client: Client<HttpConnector>,
) -> Box<Future<Item = Value, Error = hyper::Error>> {
    let uri = format!("http://registry.npmjs.org/{}/latest", name)
        .parse()
        .unwrap();
    println!("Downloading {}", uri);
    let work = client.get(uri).and_then(concat_body).map(to_json);
    Box::new(work)
}

fn concat_body(res: Response) -> Concat2<Body> {
    res.body().concat2()
}

fn to_json(body: Chunk) -> Value {
    serde_json::from_slice(&body)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))
        .unwrap()
}
