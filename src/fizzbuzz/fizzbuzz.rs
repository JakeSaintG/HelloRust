pub fn print_fizzbuzz() {
    println!("\nWould you like me print out the answer or show you how to do it in Rust?");
    
    println!("  1. Print out FizzBuzz.");
    println!("  2. Show me FizzBuzz Code.");
    println!("  3. Wait...what's FizzBuzz?");
    // Need to add a return to main fn option

    let mut response = String::new();
    match std::io::stdin().read_line(&mut response) {
        Ok(_) => {

            if response.contains("1") {
                fizzbuzz();
            } else if response.contains("2") {
                print_code();
            } else if response.contains("3") {
                print_instructions();
            } else {
                println!("Returning to main program.");
            }
        },
        Err(error) => {
            println!("Not a valid input.")
        },
    };

}

fn print_instructions () {
    println!("FizzBuzz is a coding challenge commonly used in interviews. It is also a great self-challenge for those learning how to code or for those learning a new development language.");
    println!("\nInstructions=============================================================");
    println!("Write a program that prints the numbers from 1 to 100. But for multiples of three print \"Fizz\" instead of the number and for the multiples of five print \"Buzz\". For numbers which are multiples of both three and five print \"FizzBuzz\".");
    println!("==========================================================================");
    println!("\nThere a lot of ways to solve FizzBuzz and some devs have even gotten it down to some (hard to read) one-liners. Try your hand at it some time!")
}

fn print_code () {
    println!("\nRust Code=================================================================");
    println!("for i in 1..101 {{");
    println!("    if i % 15 == 0 {{");
    println!("        println!(\"FizzBuzz\");");
    println!("    }} else if i % 3 == 0 {{");
    println!("        println!(\"Fizz\");");
    println!("    }} else if i % 5 == 0 {{");
    println!("        println!(\"Buzz\");");
    println!("    }} else {{");
    println!("        println!(\"{{}}\", i);");
    println!{"    }}"}

    println!("There's a lot of ways to solve FizzBuzz! Give it a shot sometime.")
}

fn fizzbuzz() {
    for i in 1..101 {
        if i % 15 == 0 {
            println!("FizzBuzz");
        } else if i % 3 == 0 {
            println!("Fizz");
        } else if i % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{}", i);
        }
    }
}