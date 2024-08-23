use reqwest;
use scraper::{Html, Selector};
use tokio;

#[derive(Debug)]
struct Product {
    price: String,
    name: String,
    description: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let scraped =
        scrape_html("https://webscraper.io/test-sites/e-commerce/allinone".to_string()).await?;
    search_scraped_html(scraped).await?;

    Ok(())
}

async fn scrape_html(url: String) -> Result<String, Box<dyn std::error::Error>> {
    let body = reqwest::get(url).await?.text().await?;
    Ok(body)
}

async fn search_scraped_html(raw_scraped_data: String) -> Result<(), Box<dyn std::error::Error>> {
    let mut products = Vec::<Product>::new();

    let fragment = Html::parse_document(&raw_scraped_data);
    let div_selector = Selector::parse(".caption").unwrap();

    for element in fragment.select(&div_selector) {
        let mut p_text: Vec<_> = element.text().collect();

        p_text.retain(|text| !text.trim().is_empty());

        if p_text.len() == 3 {
            let product = Product {
                price: p_text[0].to_string(),
                name: p_text[1].to_string(),
                description: p_text[2].to_string(),
            };
            products.push(product);
        } else {
            println!("Unexpected number of <p> tags, skipping...");
        }
    }

    println!("Products: {:#?}", products);

    Ok(())
}
