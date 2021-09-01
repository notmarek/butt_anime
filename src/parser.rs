
use regex::Regex;

use crate::downloader::download_file;

pub async fn squash_that_shit(that_shit: String) -> String {
    that_shit.replace(" ", "").replace("\n", "").replace("\t", "")
}

pub async fn find_the_dl_in_the_bullshit(the_bullshit: String) -> String {
    let re = Regex::new("dowloads\"><ahref=\"(.*?)\"target").unwrap();
    re.captures(&the_bullshit).unwrap()[1].to_string()
}

pub async fn find_the_dl_in_the_dl_from_the_bullshit(the_dl_from_the_bullshit: String) -> String {
    let re = Regex::new("\"dowload\"><ahref=\"(.*?)\"download").unwrap();
    re.captures(&the_dl_from_the_bullshit).unwrap()[1].trim().to_string()
}

pub async fn run_bullshit_on_gaygayanime(folder: String, gaygayanime_site_text: String, client: &reqwest::Client, ep: u64) {
    let squashed_shit = squash_that_shit(gaygayanime_site_text).await;
    let the_dl = find_the_dl_in_the_bullshit(squashed_shit).await;
    println!("{}", the_dl);
    let the_dl_from_the_bullshit = client.get(the_dl).send().await.unwrap().text().await.unwrap();
    let the_the_dl = find_the_dl_in_the_dl_from_the_bullshit(squash_that_shit(the_dl_from_the_bullshit).await).await;
    println!("{}", the_the_dl);
    download_file(client, &the_the_dl, &format!("{}/episode {}.mp4", &folder, ep)).await.unwrap();
}

pub async fn loop_over_episode_shit_nigger(aniem_id: String) {
    std::fs::create_dir(&aniem_id).unwrap_or(());
    let mut episode_gay_cunny = 1;
    let client = reqwest::Client::new();
    loop {
        let url = format!("https://gogoanime.pe/{}-episode-{}", &aniem_id, episode_gay_cunny);
        episode_gay_cunny = episode_gay_cunny + 1;
        let r = client.get(url).send().await.unwrap().text().await.unwrap();
        if r.contains(">404</h1>") {
            println!("DOwnlaod done.");
            break;
        }
        run_bullshit_on_gaygayanime(aniem_id.clone(), r, &client, episode_gay_cunny - 1).await;
        
    } 
    ()
}