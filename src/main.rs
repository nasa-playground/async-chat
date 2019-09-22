mod client;
mod server;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

fn main() -> Result<()> {
    let mut args = std::env::args();
    match (args.nth(1).as_ref().map(String::as_str), args.next()) {
        (Some("client"), None) => client::run(),
        (Some("server"), None) => server::run(),
        _ => Err("Usage: a-chat [client|server]")?,
    }
}
