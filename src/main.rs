extern crate futures;
extern crate tokio_core;
extern crate tokio_io;

use futures::{Future, Stream};
use tokio_io::{io, AsyncRead};
use tokio_core::net::TcpListener;
use tokio_core::reactor::Core;

fn main() {
    let mut core = Core::new().unwrap();
    let handle = core.handle();

    let addr = "127.0.0.1:12345".parse().unwrap();
    let tcp = TcpListener::bind(&addr, &handle).unwrap();

    let server = tcp.incoming().for_each(|(tcp, _)| {
        let (reader, writer) = tcp.split();

        let bytes_copied = io::copy(reader, writer);

        let handle_conn = bytes_copied.map(|(n, _, _)| {
            println!("wrote {} bytes", n)
        }).map_err(|err| {
            eprintln!("IO error {:?}", err)
        });

        handle.spawn(handle_conn);

        Ok(())
    });

    core.run(server).unwrap();
}
