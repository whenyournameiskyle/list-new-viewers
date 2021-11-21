use chrono::Local;
use serde::Deserialize;
use serde_json::from_str;
use std::collections::HashMap;
use std::{thread, time};

#[derive(Debug, Deserialize)]
struct Response {
    chatters: Chatters,
}
#[derive(Debug, Deserialize)]
struct Chatters {
    broadcaster: Vec<String>,
    moderators: Vec<String>,
    staff: Vec<String>,
    viewers: Vec<String>,
    vips: Vec<String>,
}

#[async_std::main]
async fn main() -> Result<(), http_types::Error> {
    println!("Starting program...");
    println!();

    let should_highlight = true;
    let should_list_new = false;
    let channels: Vec<String> = vec!["".to_owned()];
    let highlight_list: Vec<String> = vec!["".to_owned()];
    let ignore_list: Vec<String> = vec!["streamelements".to_owned()];

    let mut previous_users: HashMap<String, Vec<String>> = HashMap::new();
    let main_delay = time::Duration::from_secs(60);
    let inner_delay = time::Duration::from_secs(2);

    for channel in &channels {
        previous_users.insert(channel.clone(), vec![]);
    }

    loop {
        for channel in &channels {
            let uri = format!("https://tmi.twitch.tv/group/user/{}/chatters", channel);
            let string: String = surf::get(uri).recv_string().await?;
            let resp: Response = from_str(&string)?;
            let chatters = resp.chatters;
            let all: Vec<String> = [
                chatters.broadcaster,
                chatters.moderators,
                chatters.staff,
                chatters.viewers,
                chatters.vips,
            ]
            .concat();

            let old_previous_users = previous_users.get(&channel.clone()).unwrap();

            // filter out ignore_list
            let filtered: Vec<String> = all
                .clone()
                .into_iter()
                .filter(|chatter| !ignore_list.contains(chatter))
                .collect();

            // filter out old_previous_users
            let mut filtered: Vec<String> = filtered
                .clone()
                .into_iter()
                .filter(|chatter| !old_previous_users.contains(chatter))
                .collect();

            filtered.sort();
            previous_users.insert(channel.to_string(), all);

            let now = Local::now().format("%F %r").to_string();

            // highlight any found highlighters
            if should_highlight && !highlight_list.is_empty() {
                let to_highlight: Vec<String> = filtered
                    .clone()
                    .into_iter()
                    .filter(|chatter| highlight_list.contains(chatter))
                    .collect();

                if !to_highlight.is_empty() {
                    println!("********** {:?} {}\n {:?}", now, channel, to_highlight);
                    println!();
                }
            }

            // we have new viewers for this channel! print them to console!
            if should_list_new && !filtered.is_empty() {
                println!("{:?} {}\n {:?}", now, channel, filtered);
                println!();
            }
            thread::sleep(inner_delay);
        }
        thread::sleep(main_delay);
    }
}
