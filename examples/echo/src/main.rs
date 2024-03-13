/// First, start the sample echo-server with the following command:
/// ```sh
/// $ cargo run --bin echo
/// ```
/// Then, send a message from ipctl:
/// ```sh
/// $ cargo run --bin ipctl Hello
/// ```
#[tokio::main]
async fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let addr = "127.0.0.1:60001".parse()?;
    ipctl::Server::new(|x| x.to_string()).serve(addr).await?;
    Ok(())
}
