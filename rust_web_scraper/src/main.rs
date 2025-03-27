//run this bad boy by creating a int term in src, and using cargo run
//Commands run to initailize the project before any actual programming
//cargo new project_name
//--created a new project (in this case rust_web_scraper)
//--along with the Cargo.toml file and src directory (holds the project)
//cargo build
//--compiles the app, required every time (I think)
//cargo run
//--runs the app
//cargo add scraper reqwest --features "reqwest/blocking"
//--adds the reqwest and scaper libraries, reqwest and scraper
//---scraper: parser API that travels and manipulates HTML docs
//---reqwest: HTTP client enabling web requests in Rust, (uses proxies, cookies, custom headers)
//cargo add csv
//--download the csv library

//product structure, UNQ2tutorial
struct Product {
    url: Option<String>,
    image: Option<String>,
    name: Option<String>,
    price: Option<String>,
}

fn main() {
    //println!("Hello, world!");

    //create a dynamic array of Products UNQ2tutorial
    let mut products: Vec<Product> = Vec::new();

    //download your html doc via reqwest library
    //tutorial given link: https://www.scrapingcourse.com/ecommerce/
    //fallout link... https://fallout.bethesda.net/en
    let response = reqwest::blocking::get("https://www.scrapingcourse.com/ecommerce/");
    
    //unwrap the html content, then print it
    let html_content = response.unwrap().text().unwrap();
    println!("{}", html_content);


    //created code
    let html_test_path = std::path::Path::new("testerfile.csv");
    let mut test_writer = csv::Writer::from_path(html_test_path).unwrap();

    test_writer.write_record(&[html_content.as_str()]).unwrap();



    //extracts html from the string to the html tree/scraper
    let document = scraper::Html::parse_document(&html_content);

    //parse() from selector to define a scraper selector object
    let html_product_selector = scraper::Selector::parse("li.product").unwrap();
    let html_products = document.select(&html_product_selector);

    //iterate through the product elements, scrape elements of interest
    //UNQ2tutorial
    for html_product in html_products {
        // scraping logic to retrieve the info
        // of interest
        let url = html_product
            .select(&scraper::Selector::parse("a").unwrap())
            .next()
            .and_then(|a| a.value().attr("href"))
            .map(str::to_owned);
        let image = html_product
            .select(&scraper::Selector::parse("img").unwrap())
            .next()
            .and_then(|img| img.value().attr("src"))
            .map(str::to_owned);
        let name = html_product
            .select(&scraper::Selector::parse("h2").unwrap())
            .next()
            .map(|h2| h2.text().collect::<String>());
        let price = html_product
            .select(&scraper::Selector::parse(".price").unwrap())
            .next()
            .map(|price| price.text().collect::<String>());
    
        // instantiate a new product
        // with the scraped data and add it to the list
        let product = Product {
            url,
            image,
            name,
            price,
        };
        products.push(product);
    }
    
    //convert your PRODUCTS into a csv file
    // create the CSV output file
    let path = std::path::Path::new("products.csv");
    let mut writer = csv::Writer::from_path(path).unwrap();


    // append the header to the CSV
    writer
        .write_record(&["url", "image", "name", "price"])
        .unwrap();
    // populate the output file
    for product in products {
        let url = product.url.unwrap();
        let image = product.image.unwrap();
        let name = product.name.unwrap();
        let price = product.price.unwrap();
        writer.write_record(&[url, image, name, price]).unwrap();
    }

    // free up the resources
    writer.flush().unwrap();

   // println!("Extracted products count: {}", products.len());

}

