use rand::Rng; //random number gerenerator
use std::cmp::Ordering;
use std::io; //to obtain user input and print result as output
fn main() {
    println!("Guess the number!");

    let secret_number: u32 = rand::rng().random_range(1..=100); //start..=end inclusive
    loop {
        println!("Please input your guess.");

        let mut guess = String::new(); //initialised mutable variable [String::new => fn that returns a  a new instance of string]

        io::stdin()
            .read_line(&mut guess) //give a mutable refernce, refrences are immutable by default
            .expect("Failed to read input"); //if the instance cause an Err val, expect will crash the program and return the msg that we have passed

        let guess: u32 =match guess.trim().parse(){ //shadowing uses te same variable name. now the older one will not be used for printinhg
            Ok(num) => num,
            Err(_)=>{
                println!("Plesae Enter a Valid Input!");
                continue;
            },
        };
        match guess.cmp(&secret_number) {
            //will have to cover all the cases ie, cannot stop at equal have to mention what to do when num is less or greater also
            Ordering::Equal => {println!("You Won!"); break;},
            Ordering::Less => println!("Too Less!"),
            Ordering::Greater => println!("Too Big!"),
        }
    }
    println!("Thank You! Bye");
}
