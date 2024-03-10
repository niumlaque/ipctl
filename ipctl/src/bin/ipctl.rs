use clap::Parser;

/// ipctl - Send commands to other applications through IPC.
#[derive(Debug, Parser)]
pub struct Cli {
    /// The value to be sent to the ipctl server.
    value: String,

    /// Specify the destination for sending commands via communication.
    #[arg(
        short = 'd',
        long,
        value_name = "DESTINATION",
        default_value_t = String::from("http://127.0.0.1:60001")
    )]
    destination: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let mut client = ipctl::ControlClient::connect(cli.destination).await?;
    let request = tonic::Request::new(ipctl::Request { value: cli.value });
    let response = client.call(request).await?;
    println!("{}", response.get_ref().value);
    Ok(())
}
