use colored::*;

fn main() {
    let range: i32 = loop {
        let mut line = String::new();
        println!("{} Enter the range: ", "?".blue());
        std::io::stdin().read_line(&mut line).expect("Failed to read line");
        match line.trim().parse::<i32>() {
            Ok(r) => {
                println!("{{ Range: {} }}", r.to_string().blue().bold());
                break r;
            }
            Err(_) => { eprintln!("{}", "Invalid number — please try again.".red()); }
        }
    };
    let fizz = "Fizz".green().bold();
    let buzz = "Buzz".bright_yellow().bold();
    for i in 1..range+1 {
        if i % 3 == 0 && i % 5 == 0 {
            println!("{}{}", fizz, buzz);
        } else if i % 3 == 0 {
            println!("{}", fizz);
        } else if i % 5 == 0 {
            println!("{}", buzz);
        } else {
            println!("{}", i.to_string().blue().italic());
        }
    }
}
