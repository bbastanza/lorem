use reqwest::Error;

async fn fetch_lorem() -> Result<Vec<String>, Error> {
    let url = "https://baconipsum.com/api/?type=all-meat&paras=3&start-with-lorem=0&format=text";
    let response = reqwest::get(url).await?.text().await?;

    let words: Vec<&str> = response.split_ascii_whitespace().collect();

    let x = words
        .iter()
        .map(|word| word.chars().filter(|c| c.is_alphabetic()).collect())
        .collect();

    Ok(x)
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().collect();

    let mut output_count: i32 = 30;

    if args.len() == 2 {
        let result = args[1].parse();
        match result {
            Ok(count) => output_count = count,
            _ => (),
        }
    }

    let mut word_count: i32 = 0;

    while output_count > word_count {
        let lorem = fetch_lorem().await.unwrap();

        for word in lorem {
            word_count = word_count + 1;

            if word_count > output_count {
                break;
            }

            if word == "" {
                continue;
            }

            print!("{} ", word);
        }
    }
}
