use std::io::prelude::*;
use std::fs::File;
use std::io::Error;

fn main() {
    let initial: Vec<Colour> = vec![
    Colour::White,
    Colour::Black,
    Colour::Orange,
    Colour::Purple,
    Colour::Green,
    Colour::Red
    ];

    let chosen_colours: Vec<Colour> = read_file().map(str_to_vec).expect("this is a dumb fucking test, the file is always there");

    let mut prev = initial;
    for colour in chosen_colours {
        prev = if let Some(x) = prev.iter().find(|c| **c == colour) {
            match colour {
                Colour::White => vec![Colour::Green, Colour::Orange, Colour::Purple, Colour::Red, ],
                Colour::Red => vec![Colour::Green, ],
                Colour::Black => vec![Colour::Red, Colour::Purple, Colour::Black, ],
                Colour::Orange => vec![Colour::Red, Colour::Black, ],
                Colour::Green => vec![Colour::Orange, Colour::White, ],
                Colour::Purple => vec![Colour::Black, Colour::Red, ],
            }
        } else {
            println!("boom");
            return;
        }
    }

    println!("you defused the bomb.")
}

fn read_file() -> Result<Vec<String>, Error> {
    let mut file = File::open("res/input2.txt")?;
    let mut string = String::new();
    file.read_to_string(&mut string)?;

    Ok(string.lines().map(|str| str.into()).collect())
}

fn str_to_vec(input: Vec<String>) -> Vec<Colour> {
    input.iter().map(|s| s.as_ref()).filter_map(|s| {
        match s {
            "White" => Some(Colour::White),
            "Black" => Some(Colour::Black),
            "Orange" => Some(Colour::Orange),
            "Purple" => Some(Colour::Purple),
            "Green" => Some(Colour::Green),
            "Red" => Some(Colour::Red),
            _ => None,
        }
    }).collect()
}

#[derive(Eq, PartialEq)]
enum Colour {
    White,
    Black,
    Orange,
    Purple,
    Green,
    Red,
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
