

use crate::graph::run;
use std::io::Error;

pub fn spawn() -> Result<(), Error> {
    
    println!("test");
        run().unwrap_or_else(|error| {
            println!("{}", error);
        }); 
    
    
    Ok(())
}