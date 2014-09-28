use std::io;
use std::rand;

enum Ordering {
    Less,
    Greater,
    Equal,
}

fn main() {
    println!("I've picked a number, try to guess it!!");
    let secret_number = rand100();
    let mut past_guesses = vec![];
    loop {
        println!("Past guesses: {}", past_guesses.connect(", "));
        let guess = get_input();
        past_guesses.push(guess.to_string());
        match compare(guess, secret_number) {
            Less => println!("Too low"),
            Greater => println!("Too high"),
            Equal => {
                println!("You got it in {} guesses!", past_guesses.len());
                break;
            }
        }
    }
}

fn get_input() -> uint {
    loop {
        print!("Please enter a number: ");
        let input = io::stdin().read_line()
                               .ok()
                               .expect("Could not read line");
        let input_num: Option<uint> = from_str(input.as_slice().trim());
        match input_num {
            Some(num) => {
                println!("You guessed: {}", num);
                return num;
            },
            None => { continue; }
        }
    }
}

fn compare(a: uint, b: uint) -> Ordering {
    if a < b { Less }
    else if a > b { Greater }
    else { Equal }
}

fn rand100() -> uint {
    let random = rand::random::<uint>() % 100u;
    random + 1u
}
