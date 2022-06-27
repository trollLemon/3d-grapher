mod graph;
use std::io::Error;

pub fn spawn(x :String, y:String, z:String) -> Result<(), Error> {
    graph::run(x,y,z).unwrap_or_else(|error| {
        println!("um:{}", error);
    });

    Ok(())
}
