use csv::WriterBuilder;
use serde::Serialize;
use std::fs::OpenOptions;
use std::io::Seek;
use std::path::Path;

#[derive(Debug, Serialize)]
struct Quote {
    quote: String,
    author: String
}

fn main() -> Result<(), ureq::Error> {
    let file_path = Path::new("quotes.csv");

    for page_no in 1..6 {
        let url = format!("https://quotes.toscrape.com/page/{}/", page_no);
        let response = ureq::get(url.as_str()).call()?.into_string();

        let document = scraper::Html::parse_document(response?.as_str());

        let quote_selector = scraper::Selector::parse("div.quote>span.text").unwrap();
        let author_selector = scraper::Selector::parse("div.quote>span>small.author").unwrap();

        let quotes = document.select(&quote_selector)
            .map(|x| x.inner_html()).collect::<Vec<String>>();

        let authors = document.select(&author_selector)
            .map(|x1| x1.inner_html()).collect::<Vec<String>>();

        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .append(true)
            .open(file_path)?;

        let needs_headers = file.seek(std::io::SeekFrom::End(0))? == 0;

        let mut wtr = WriterBuilder::new()
            .has_headers(needs_headers)
            .from_writer(file);

        quotes.iter().zip(authors.iter()).for_each(|(quote, author)| {
            wtr.serialize(
                Quote {
                    quote: quote.to_string(),
                    author: author.to_string()
                }
            ).expect("failed to write");
        });

        wtr.flush()?;
    }

    println!("Finished writing");

    Ok(())
}
