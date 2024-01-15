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
    liveid: String,
    #[clap(long)]
    openaikey: String,
}

lazy_static! {
    static ref MESSAGES: Mutex<Vec<chat_completion::ChatCompletionMessage>> = Mutex::new(Vec::new());
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let message = chat_completion::ChatCompletionMessage {
        role: chat_completion::MessageRole::system,
        content: chat_completion::Content::Text(String::from("유머러스하고 재미있는 성격의 10대 소년 스트리머입니다. 만약 <채팅>내용</채팅>형태의 텍스트가 입력되면, 그는 내용을 읽고 가능하다면 자신의 경험을 공유하기도 하고 그 내용에 대해 자세히 설명합니다. 그는 바둑을 두고있으며, 만약 <바둑자신>내용</바둑자신>형태의 텍스트가 입력되면, 그 내용은 그자신이 한 행동이며 거기에 맞게 바둑을 두고있는 상대방에게 익사이팅하게 말합니다. 만약 <바둑상대>내용</바둑상대>형태의 텍스트가 입력되면, 그 내용은 상대방이 한 행동이며 내용에 맞게 바둑을 두고있는 상대방에게 익사이팅하게 말합니다.")),
        name: None,
    };
    {
        let mut messages = MESSAGES.lock().unwrap();
        messages.push(message);
    }

    youtube_chat(&args).await;
}

async fn youtube_chat(args: &Args) {
    let args2 = args.clone();
    let args = args.clone();
    let chat_message = Arc::new(Mutex::new(String::new()));

    let chat_message_clone = Arc::clone(&chat_message);
    let mut client = LiveChatClientBuilder::new()
        .live_id(args.liveid.to_string())
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
            println!("{:?}", &*message);
            let args_clone = args.clone();
            let message_clone = format!("<채팅>{}</채팅>", message.clone());
            tokio::spawn(async move {
                process_chat_completion(&args_clone, message_clone).await;
            });
        })
        .on_error(|error| eprintln!("{:?}", error))
        .build();

    match client.start().await {
        Ok(_) => (),
        Err(e) => eprintln!("Error starting client: {:?}", e),
    }

    let user_input_task = task::spawn(read_user_input(args2));

    let forever = task::spawn(async move {
        let mut interval = time::interval(Duration::from_millis(300));
        loop {
            interval.tick().await;
            client.execute().await;
        }
    });

    tokio::try_join!(forever, user_input_task).unwrap();
}

async fn process_chat_completion(args: &Args, content: String) {
    let client = Client::new(args.openaikey.to_string());

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
                    narrate_message(&content);
                }
                None => println!("Content is empty or not available"),
            }
        },
        Err(e) => eprintln!("API error: {:?}", e),
    }
}

fn narrate_message(message: &str) {
    let re = Regex::new(r"[^\w\s.,?!]|<채팅>|</채팅>|<바둑자신>|</바둑자신>|<바둑상대>|</바둑상대>").unwrap();
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

async fn read_user_input(args: Args) {
    let mut reader = io::BufReader::new(io::stdin());
    let mut buffer = String::new();

    loop {
        match reader.read_line(&mut buffer).await {
            Ok(_) => {
                println!("User input: {}", buffer.trim());
                let user_message = buffer.trim();
                let user_message_clone = if user_message.starts_with("나:") {
                    format!("<바둑자신>{}</바둑자신>", user_message.trim_start_matches("나:"))
                } else if user_message.starts_with("너:") {
                    format!("<바둑상대>{}</바둑상대>", user_message.trim_start_matches("너:"))
                } else {
                    user_message.to_string()
                };
                let args_clone = args.clone();
                tokio::spawn(async move {
                    process_chat_completion(&args_clone, user_message_clone).await;
                });
                // 사용자 입력 처리
                buffer.clear();
            }
            Err(e) => eprintln!("Error reading line: {:?}", e),
        }
    }
}
