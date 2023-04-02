use std::env;

mod svt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    let page = match args.len() {
        1 => &args[1],
        2 => &args[1],
        _ => panic!("Too many params!"),
    };

    let client = svt::SvtClient::new(String::from("https://www.svt.se/text-tv/api"));
    let result = client.get_page(String::from(page)).await;
    let resp = match result {
        Ok(result) => result,
        Err(error) => panic!("Error getting page: {:?}", error),
    };

    for p in resp.data.sub_pages {
        println!("{}", p.alt_text)
    }
    Ok(())
}
