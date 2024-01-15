# AI-Vtuber
인공지능으로 작동하는 바둑 버튜버 개발을 목표로 하고있습니다.
이 코드는 YouTube의 채팅 메시지를 읽은 다음 OpenAI의 GPT-4 모델을 사용하여 응답을 생성하도록 설계할 예정입니다.
GPT-4의 출력은 ElevenLabs에서 제공하는 TTS엔진을 사용하여 읽도록 합니다.


# Setup
Install dependencies
```
git clone https://github.com/GG-O-BP/ai_korean_go_vtuber.git
cd ai_korean_go_vtuber
cargo build
```

# Usage
```
cargo run -- --liveid "스트림id"
```

## Notes
"스트림id"를 실제 라이브 스트림의 id로 입력해주세요.

Replace `STREAMID` with the stream's ID that you can find on the Youtube Stream link

# Other

- [x] rust로 수정
- [x] youtube 라이브채팅을 읽음
- [x] 읽은 채팅을 tts로 출력
- [x] 읽은 채팅으로 Chat-GPT 응답을 생성
- [ ] 바둑의 해설첨삭에 따른 대사 생성
- [ ] 바둑의 수에 따른 대사 생성
- [ ] 바둑AI로 응수를 생성
- [ ] 바둑AI를 통한 참고도 생성
- [ ] 바둑AI와 chat-GPT를 조합한 해설 생성

# License
This program is under the [MIT license](/LICENSE) 
