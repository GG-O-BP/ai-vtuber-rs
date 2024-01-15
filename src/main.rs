use clap::Parser;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::{task, time};
use youtube_chat::live_chat::LiveChatClientBuilder;
use youtube_chat::item::MessageItem;
use tts_rust::tts::GTTSClient;
use tts_rust::languages::Languages;

#[derive(Parser, Debug)]
struct Args {
    #[clap(long)]
    liveid: String,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    youtube_chat(&args.liveid).await;
    narrate_message("메시지");
}

async fn youtube_chat(live_id: &str) {
    let chat_message = Arc::new(Mutex::new(String::new()));

    let chat_message_clone = Arc::clone(&chat_message);
    let mut client = LiveChatClientBuilder::new()
        .live_id(live_id.to_string())
        .on_chat(move |chat_item| {
            let mut message = chat_message_clone.lock().unwrap();
            match &chat_item.message[0] {
                MessageItem::Text(text_message) => {
                    *message = text_message.clone();
                },
                _ => {
                    eprintln!("Unsupported message type");
                }
            }
            println!("{:?}", *message);
            narrate_message(&*message);
        })
        .on_error(|error| eprintln!("{:?}", error))
        .build();

    match client.start().await {
        Ok(_) => (),
        Err(e) => eprintln!("Error starting client: {:?}", e),
    }

    let forever = task::spawn(async move {
        let mut interval = time::interval(Duration::from_millis(300));
        loop {
            interval.tick().await;
            client.execute().await;
        }
    });

    forever.await.unwrap();
}

fn narrate_message(message: &str) {
    let narrator: GTTSClient = GTTSClient {
        volume: 1.0, 
        language: Languages::Korean,
        tld: "com"
    };
    let _ = narrator.speak(message);
}
