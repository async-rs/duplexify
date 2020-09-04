use async_std::io::{self, prelude::*, BufReader};
use async_std::task;
use duplexify::Duplex;

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
