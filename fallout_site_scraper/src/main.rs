use std::io;

fn name_your_file() -> String{
    let mut temp_string = String::new();
    println!("\nEnter your csv file name: ");
    io::stdin().read_line(&mut temp_string).expect("Failed to read input");
    String::from(temp_string)
}

fn main() {
    //init the loop status
    let mut loop_status = true;

    //start up the menu loop
    while loop_status == true {

    
        let mut file_name = String::new();
        let mut read_string = String::new();

        
        println!("What do you want to scrape?");
        println!("1. Scrape Fallout 76 News Cards\n2. Scrape Game images and titles\n3. Scrape Character Images\n4. Quit Program");
        
        io::stdin().read_line(&mut read_string).expect("Failed to read input");
        let user_number: i32 = read_string.trim().parse().expect("Invalid Number");
        match user_number{
            1 => {
                file_name = name_your_file();
                println!("Hello Mobius!");
                println!("{}", file_name);
            }
            4 => {
                loop_status = false; 
                println!("\nQuitting Program, Goodbye!\n");
            }
            _ => println!("Please input a valid option."),
        }
        
        
    //use break; to destroy
    }

    //scrape news cards, or scrape game titles w/ pictures 
}
