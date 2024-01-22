use clap::Parser;

use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::{task, time};
use tokio::io::{self, AsyncBufReadExt};

use youtube_chat::live_chat::LiveChatClientBuilder;
use youtube_chat::item::MessageItem;

use lazy_static::lazy_static;
use regex::Regex;
use openai_api_rs::v1::api::Client;
use openai_api_rs::v1::chat_completion::{self, ChatCompletionRequest};
use openai_api_rs::v1::common::GPT4;

use tts_rust::tts::GTTSClient;
use tts_rust::languages::Languages;

#[derive(Parser, Debug, Clone)]
struct Args {
    #[clap(long)]
    openaikey: String,
    #[clap(long)]
    prompt: String,
    #[clap(long)]
    liveid: String,
}


lazy_static! {
    static ref MESSAGES: Mutex<Vec<chat_completion::ChatCompletionMessage>> = Mutex::new(Vec::new());
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let message = chat_completion::ChatCompletionMessage {
        role: chat_completion::MessageRole::system,
        content: chat_completion::Content::Text(String::from(args.prompt.clone())), // args.prompt를 복제합니다.
        name: None,
    };
    {
        let mut messages = MESSAGES.lock().unwrap();
        messages.push(message);
    }

    let args = Arc::new(args);
    youtube_chat(args).await;
}

async fn youtube_chat(args: Arc<Args>) {
    let args_clone_for_chat = Arc::clone(&args);
    let chat_message = Arc::new(Mutex::new(String::new()));

    let chat_message_clone = Arc::clone(&chat_message);
    let mut client = LiveChatClientBuilder::new()
        .live_id(args.liveid.to_string())
        .on_chat(move |chat_item| {
            let message = {
                let mut message_guard = chat_message_clone.lock().unwrap();
                match &chat_item.message[0] {
                    MessageItem::Text(text_message) => {
                        *message_guard = text_message.clone();
                    },
                    _ => {
                        eprintln!("Unsupported message type");
                    }
                }
                println!("{:?}", &*message_guard);
                message_guard.clone()
            };
            let args_clone = Arc::clone(&args_clone_for_chat);
            tokio::spawn(async move {
                process_chat_completion(args_clone.openaikey.clone(), message).await;
            });
        })
        .on_error(|error| eprintln!("{:?}", error))
        .build();

    match client.start().await {
        Ok(_) => (),
        Err(e) => eprintln!("Error starting client: {:?}", e),
    }

    let user_input_task = task::spawn(read_user_input(args.openaikey.clone()));

    let forever = task::spawn(async move {
        let mut interval = time::interval(Duration::from_millis(300));
        loop {
            interval.tick().await;
            client.execute().await;
        }
    });

    tokio::try_join!(forever, user_input_task).unwrap();
}

async fn process_chat_completion(openaikey: String, content: String) {
    let client = Client::new(openaikey);

    let message = chat_completion::ChatCompletionMessage {
        role: chat_completion::MessageRole::user,
        content: chat_completion::Content::Text(String::from(content)),
        name: None,
    };
    {
        let mut messages = MESSAGES.lock().unwrap();
        messages.push(message);
    }

    let messages = MESSAGES.lock().unwrap().to_vec();

    let req = ChatCompletionRequest::new(
        GPT4.to_string(),
        messages,
    );

    let result = client.chat_completion(req);
    match result {
        Ok(response) => {
            match &response.choices[0].message.content {
                Some(content) => {
                    println!("Content: {}", content);
                    let response_message = chat_completion::ChatCompletionMessage {
                        role: chat_completion::MessageRole::assistant,
                        content: chat_completion::Content::Text(String::from(content)),
                        name: None,
                    };
                    {
                        let mut messages = MESSAGES.lock().unwrap();
                        messages.push(response_message);
                    }
                    narrate_message(&content);
                }
                None => println!("Content is empty or not available"),
            }
        },
        Err(e) => eprintln!("API error: {:?}", e),
    }
}

fn narrate_message(message: &str) {
    let re = Regex::new(r"[^\w\s.,?!\-]").unwrap();
    let cleaned_message = re.replace_all(message, "");
    let parts: Vec<&str> = cleaned_message.split(|c| c == '.' || c == ',' || c == '!' || c == '?').collect();

    let narrator: GTTSClient = GTTSClient {
        volume: 1.0, 
        language: Languages::Korean,
        tld: "com"
    };
    for part in parts {
        let _ = narrator.speak(part);
    }
}

async fn read_user_input(openaikey: String) {
    let mut reader = io::BufReader::new(io::stdin());
    let mut buffer = String::new();

    loop {
        match reader.read_line(&mut buffer).await {
            Ok(_) => {
                println!("User input: {}", buffer.trim());
                let user_message = buffer.trim().to_string();
                let openaikey_clone = openaikey.clone();
                tokio::spawn(async move {
                    process_chat_completion(openaikey_clone, user_message).await;
                });
                // 사용자 입력 처리
                buffer.clear();
            }
            Err(e) => eprintln!("Error reading line: {:?}", e),
        }
    }
}
