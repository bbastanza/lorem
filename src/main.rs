use reqwest::Error;

async fn fetch_lorem() -> Result<Vec<String>, Error> {
    let url =
        "https://baconipsum.com/api/?type=meat-and-filler&paras=20&start-with-lorem=0&format=text";

    let response = reqwest::get(url).await?.text().await?;

    let words: Vec<String> = response
        .split_ascii_whitespace()
        .map(|w| w.to_string() + " ")
        .collect();

    Ok(words)
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().collect();

    let mut output_count = 30;

    if args.len() == 2 {
        let result = args[1].parse();
        match result {
            Ok(count) => output_count = count,
            _ => (),
        }
    }

    let mut word_count = 0;
    let mut output = String::from("");

    while output_count > word_count {
        let lorem = fetch_lorem().await.unwrap();

        for word in lorem {
            if word_count == output_count {
                break;
            }

            word_count = word_count + 1;

            output = output + &word;
        }
    }

    println!("{}", output.trim())
}
