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
2. git，檢查系統是否有安裝homebrew，如果沒有顯示訊息「需要先安裝git」並且回到主選單，如果有，檢查系統是否有安裝git，如果有，回到主選單。如果沒有，幫使用者安裝。(安裝指令：`brew install git`)
3. nvm，檢查系統是否有安裝nvm，如果有，回到主選單。如果沒有，幫使用者安裝。(安裝指令：`curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.40.3/install.sh | bash`)

## Tests
1. homebrew
   1. 可以正確辨識是否有安裝
   2. 可以正確顯示版本
   3. 可以幫使用者安裝
   4. 安裝完可以確認版本
2. git
   1. 可以正確辨識是否有安裝homebrew
   2. 可以正確辨識是否有安裝git
   3. 可以正確顯示git版本
   4. 可以幫使用者安裝git
   5. 安裝完可以確認git版本
3. nvm
   1. 可以正確辨識是否有安裝
   2. 可以正確顯示版本
   3. 可以幫使用者安裝
   4. 安裝完可以確認版本