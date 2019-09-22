use async_std::{
    io::{stdin, BufRead, BufReader, Write},
    net::{TcpStream, ToSocketAddrs},
    task,
};
use futures::{select, StreamExt};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub fn run() -> Result<()> {
    task::block_on(try_run("127.0.0.1:8080"))
}

async fn try_run(addr: impl ToSocketAddrs) -> Result<()> {
    let stream = TcpStream::connect(addr).await?;
    let (reader, mut writer) = (&stream, &stream);

    let reader = BufReader::new(reader);
    let stdin = BufReader::new(stdin());

    let mut lines_from_server = futures::StreamExt::fuse(reader.lines());
    let mut lines_from_stdin = futures::StreamExt::fuse(stdin.lines());

    loop {
        select! {
            line = lines_from_server.next() => match line {
                None => break,
                Some(line) => {
                    let line = line?;
                    println!("{}", line);
                }
            },
            line = lines_from_stdin.next() => match line {
                None => break,
                Some(line) => {
                    let line = line?;
                    writer.write_all(line.as_bytes()).await?;
                    writer.write_all(b"\n").await?;
                }
            }
        }
    }

    Ok(())
}
