use std::io::prelude::*;
use std::fs::File;

fn main() {
    println!("helloWorld")
}

fn read_fle() -> Result<Vec<String, io::Error>> {
     File::open("res/input.txt")?
}



#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
