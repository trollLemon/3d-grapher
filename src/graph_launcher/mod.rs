mod graph;
use std::io::Error;

pub fn spawn() -> Result<(), Error> {
    graph::run().unwrap_or_else(|error| {
        println!("{}", error);
    });

    Ok(())
}
