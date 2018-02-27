extern crate flate2;
extern crate futures;
extern crate hyper;
extern crate serde;
extern crate serde_json;
extern crate tar;
extern crate tokio_core;
extern crate rhino;

use rhino::downloader;
use tokio_core::reactor::Core;
use hyper::Client;
use futures::Future;

fn main() {
    let start = std::time::Instant::now(); 

    let mut core = Core::new().unwrap();
    let client = Client::new(&core.handle());
    let down_work = downloader::download_package("imat", client.clone());
    let another = downloader::download_package("chalk", client.clone());
    core.run(down_work.join(another)).unwrap();

    println!("{:?}", start.elapsed());
}
