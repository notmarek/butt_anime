mod downloader;
mod parser;
#[tokio::main]
async fn main() {
    let mut input_string = String::new();
    println!("give me gogo anime id pls");
    std::io::stdin()
        .read_line(&mut input_string)
        .ok()
        .expect("Failed to read line");

    parser::loop_over_episode_shit_nigger(input_string.trim().to_string()).await;
    ()
}
