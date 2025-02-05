use poem::{
    handler,
    listener::{Listener, TcpListener},
    Result, Route, Server,
};

#[handler]
fn hello() -> String {
    "Hello from poem!\n".to_string()
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let app = Route::new().at("/", poem::get(hello));
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "poem=debug");
    }
    tracing_subscriber::fmt::init();

    // To test port assignment, run two instances of this example at once.
    //
    // For ports <1024, running with administrator priveleges would be needed on
    // Unix. For port 0, the OS would assign a port and we'd need to find out
    // what that port's number is.
    let (min_port, max_port) = (8080, 8085);
    // Using 127.0.0.1 instead of 0.0.0.0 for security; a local server should
    // not, generally, be visible from the network.
    let hostname = "127.0.0.1";
    let mut port = min_port;
    let mut error = None;
    let acceptor = loop {
        if port > max_port {
            return Err(error.unwrap());
        }
        let listener = TcpListener::bind(format!("{hostname}:{}", port));
        match listener.into_acceptor().await {
            Ok(a) => break a,
            Err(err) => {
                // Most likely, another application is bound to this port.
                eprintln!("Couldn't bind to port {port}.");
                error = Some(err)
            }
        };
        port += 1;
    };

    // Now that the acceptor exists, the browser should be able to connect
    let http_address = format!("http://{hostname}:{port}/");
    eprintln!("Listening at {http_address}.");
    eprint!("Trying to launch a browser at {http_address}...");
    match open::that(&http_address) {
        Ok(_) => eprintln!(" Success!"),
        Err(err) => eprintln!("\nFailed to launch a browser: {err}"),
    }

    Server::new_with_acceptor(acceptor).run(app).await?;
    Ok(())
}
