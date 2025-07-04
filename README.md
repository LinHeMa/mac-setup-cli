## Architecture
```
my-rust-cli
├── Cargo.lock
├── Cargo.toml
├── src
│   └── lib.rs
└── tests
    └── homebrew.rs
```

## Features
1. homebrew，檢查系統是否有安裝homebrew，如果有，回到主選單。如果沒有，幫使用者安裝。(安裝指令：`/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"`)
2. git，檢查系統是否有安裝git，如果有，回到主選單。如果沒有，幫使用者安裝。(安裝指令：`brew install git`)
3. nvm，檢查系統是否有安裝nvm，如果有，回到主選單。如果沒有，幫使用者安裝。(安裝指令：`brew install nvm`)

## Tests
1. homebrew
   1. 可以正確辨識是否有安裝
   2. 可以正確顯示版本
   3. 可以幫使用者安裝
   4. 安裝完可以確認版本
2. git
   1. 可以正確辨識是否有安裝
   2. 可以正確顯示版本
   3. 可以幫使用者安裝
   4. 安裝完可以確認版本