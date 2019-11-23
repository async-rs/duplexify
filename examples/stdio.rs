use async_std::io::{self, BufReader, prelude::*};
use duplexify::Duplex;
use async_std::task;

fn main() -> std::io::Result<()> {
    task::block_on(async {
        let stdin = BufReader::new(io::stdin());
        let stdout = io::stdout();
        let mut stdio = Duplex::new(stdin, stdout);

        let mut line = String::new();
        stdio.read_line(&mut line).await?;
        stdio.write_all(&line.as_bytes()).await?;

        Ok(())
    })
}
