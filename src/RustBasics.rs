mod learnxiny;
mod fizzbuzz;

#[allow(unused_variables)]
#[allow(unused_assignments)]
#[allow(dead_code)]
fn main() {
    
    println!("Hello, world!");
    println!("\n===================================================================");

    let mut stay_open: bool = true;

    while stay_open {

        let mut line = String::new();
        println!("\nProvide a number for what you would like me to do...");
        println!("  1. Do me fizzbuzz");
        println!("  2. Add some stuff");
        println!("  3. Call me a nerd");

        match std::io::stdin().read_line(&mut line) {
            Ok(_) => {

                if line.contains("1") {
                    fizzbuzz::fizzbuzz::print_fizzbuzz();
                } else if line.contains("2") {
                    let mut y: i32 = 13i32;
                    let x: f64 = 1.3f64;
        
                    //reassigned just for fun because I want to learn
                    y = 8; 
                    let z: f64 = learnxiny::learnxiny::add_2(x, y);
        
                    println!("\nLooks like {} plus {} equals {} ", y, x, z);
                } else if line.contains("3") {
                    println!("\nUhh...okay? Nerd");
                } else {
                    println!("\nFam...you gotta gimme something here.");
                }

            },
            Err(error) => {
                println!("Try again... Not a valid input.")
            },
        };



        println!("\n\nWould you like to continue?");
        line = String::new();
        match std::io::stdin().read_line(&mut line) {
            Ok(_) => {
                let test: &str = &line.to_lowercase();
                let test = test.trim();
        
                match test {
                    "yes" | "true" | "continue" | "go" => println!("Continuing..."),
                    "no" | "false" | "stop" => {
                        println!("Stopping...");
                        stay_open = false;
                    },
                    _ => println!("I'm not really sure what you want from me soo...CONTINUING!")
                };
            },
            Err(error) => {
                println!("Try again... Not a valid input.")
            },
        };
    }
}