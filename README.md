# palheui

palheui(빨희)는 빠른 아희 코드 실행을 목표로 하는 아희-to-C 트랜스파일러입니다.

## 정확성

palheui는 aheui-snippet의 전체 57개 테스트 케이스 중 64bit.aheui를 제외한 56개를 통과합니다.

## 설치

```bash
$ git clone https://github.com/kiwiyou/palheui.git
$ cd palheui
$ cargo install .
$ palheui
```

## 사용법

`palheui [INPUT]` - 아희 코드를 파일 `[INPUT]`에서 불러와 C로 트랜스파일한 결과를 표준 출력 스트림에 출력합니다.

