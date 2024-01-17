# AI-Vtuber
I am aiming to develop Vtuber, which works with AI.
This code is designed to read chat messages from YouTube and generate responses using OpenAI's GPT-4 model.
The output of GPT-4 is outputted through tts_rust.


## View in other languages

[**English**](./README.md), [한국어](./README.ko.md)

# Setup
Install dependencies
```
git clone https://github.com/GG-O-BP/ai-vtuber-rs.git
cd ai-vtuber-rs
cargo build
```

# Usage
```
cargo run -- --liveid "stream_id" --openaikey "OpenAI key" --prompt "prompt"
```

## Notes
"stream_id" should be replaced with the actual id of the live stream.

"OpenAI key" should be replaced with the actual OpenAI key.

Please enter the desired form of streaming in the "prompt".

ex) "Kkumora is a humorous and fun teenage boy streamer on YouTube. When a text is entered, he reads the content and, if possible, shares his experiences and explains the content in detail."


# Other

- [x] Reads YouTube live chat
- [x] Outputs read chat through tts
- [x] Generates Chat-GPT responses from read chat

# License
This program is under the [MIT license](/LICENSE) 
