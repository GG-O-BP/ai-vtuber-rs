# AI-Vtuber
인공지능으로 작동하는 버튜버 개발을 목표로 하고있습니다.
이 코드는 YouTube의 채팅 메시지를 읽은 다음 OpenAI의 GPT-4 모델을 사용하여 응답을 생성하도록 설계하였습니다.
GPT-4의 출력은 tts_rust로 출력합니다.


## 다른 언어로 보기

[English](./README.md), [**한국어**](./README.ko.md)

# 설치
Install dependencies
```
git clone https://github.com/GG-O-BP/ai-vtuber-rs.git
cd ai-vtuber-rs
cargo build
```

# 사용
```
cargo run -- --liveid "스트림id" --openaikey "OpenAI key" --prompt "프롬프트"
```

## 설명
"스트림id"를 실제 라이브 스트림의 id로 입력해주세요.

"OpenAI key"에 실제 OpenAI key를 입력해주세요.

"프롬프트"에 원하는 형태의 스트리밍을 입력해주세요.

ex) "꿈모라는 이름의 유머러스하고 재미있는 성격의 10대 소년 유튜브 스트리머입니다. 텍스트가 입력되면, 그는 내용을 읽고 가능하다면 자신의 경험을 공유하기도 하고 그 내용에 대해 자세히 설명합니다."


# 기타

- [x] youtube 라이브채팅을 읽음
- [x] 읽은 채팅을 tts로 출력
- [x] 읽은 채팅으로 Chat-GPT 응답을 생성

# License
This program is under the [MIT license](/LICENSE) 
