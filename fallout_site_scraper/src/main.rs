use std::io;
use std::path::PathBuf;

fn name_your_file() -> String{
    let mut temp_string = String::new();
    println!("\nEnter your csv file name: ");
    io::stdin().read_line(&mut temp_string).expect("Failed to read input");
    String::from(temp_string)
}

fn reqwest_breakdown(sp_html: &str) -> String{
    let response = reqwest::blocking::get(sp_html);

    let html_content = response.unwrap().text().unwrap();

    String::from(html_content)
}

fn main() {
    //init the loop status
    let mut loop_status = true;

    //start up the menu loop
    while loop_status == true {

    
        let mut file_name = String::new();
        let mut read_string = String::new();
        let mut temp_full_html = String::new();

        
        println!("What do you want to scrape?");
        println!("1. Scrape Fallout 76 News Cards\n2. Scrape Game images and titles\n3. Scrape Character Images\n4. Quit Program");
        
        io::stdin().read_line(&mut read_string).expect("Failed to read input");
        let user_number: i32 = read_string.trim().parse().expect("Invalid Number");
        match user_number{
            1 => {
                file_name = name_your_file();
                temp_full_html = reqwest_breakdown("https://fallout.bethesda.net/en");
                
                //create path to directory
                let formatted_path = format!("{}.csv", file_name.trim());
                let my_path: PathBuf = PathBuf::from(formatted_path);

                //declare new writer
                let mut my_writer = csv::Writer::from_path(my_path).unwrap();
                my_writer.write_record(&[temp_full_html.as_str()]).unwrap();
            
        
                
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
