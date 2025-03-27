use std::io;


fn main() {

    println!("What's your name? ");

    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read line :(");

    let user_input = user_input.trim();

    println!("Hello {}", user_input);





    println!("Hello World!");

    let my_string = "This is a string.";

    println!("My string says: {}", my_string);

    let mut user_input = String::new();



//add a while loop to keep our menu open


    let mut loop_status: bool = true;

    while loop_status == true {

        ///csv file schenaigans
        println!("1. Write to a new my_file.txt file(deletes any existing data)\n2. Write to an existing file (appends to file)\n3. Quit Program");

        io::stdin()
                .read_line(&mut user_status)
                .expect("Failed to Read.");
        //let user_status = user_status.trim();

        if user_status == "1" {
            println!("Writing to a new file...");
        }
        else if user_status == "2"{
            println!("Appending to the expected file...");
        }
        else if user_status == "3"{
            println!("Ending Program...");
            let user_status = false;
        }


    }


} 

//compile ----> int terminal -> rustc .\file.rs (should make compiler files) --> .\main.exe to run!
//MUST be compiled during each run!


//rust web scrape TUTORIAL
//https://www.zenrows.com/blog/rust-web-scraping#is-rust-good-for-web-scraping