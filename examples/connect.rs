extern crate session_types;
#[allow(clippy::wildcard_imports)]
use session_types::*;

fn server(c: Chan<(), Eps>) {
    c.close();
}

fn client(c: Chan<(), Eps>) {
    c.close();
}

fn main() {
    connect(server, client);
}
