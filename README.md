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
2. git，檢查系統是否有安裝homebrew，如果沒有顯示訊息「需要先安裝homebrew」並且回到主選單，如果有，檢查系統是否有安裝git，如果有，回到主選單。如果沒有，幫使用者安裝。(安裝指令：`brew install git`)
3. nvm，檢查系統是否有安裝nvm，如果有，回到主選單。如果沒有，幫使用者安裝。(安裝指令：`curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.40.3/install.sh | bash`)，安裝完之後幫使用者下載最新的lts nodejs。
4. vscode，檢查系統是否有安裝homebrew，如果沒有顯示訊息「需要先安裝homebrew」並且回到主選單，如果有，檢查系統是否有安裝vscode，如果有，回到主選單。如果沒有，幫使用者安裝。安裝完打開vscode app(安裝指令：`brew install --cask visual-studio-code`)
5. google chrome，檢查系統是否有安裝homebrew，如果沒有顯示訊息「需要先安裝homebrew」並且回到主選單，如果有，檢查系統是否有安裝chrome，如果有，回到主選單。如果沒有，幫使用者安裝。安裝完打開google chrome app(安裝指令：`brew install --cask google-chrome`)
6. docker，檢查系統是否有安裝homebrew，如果沒有顯示訊息「需要先安裝homebrew」並且回到主選單，如果有，檢查系統是否有安裝docker，如果有，回到主選單。如果沒有，幫使用者安裝。安裝完打開docker app(安裝指令：`brew install --cask docker`)
7. slack，檢查系統是否有安裝homebrew，如果沒有顯示訊息「需要先安裝homebrew」並且回到主選單，如果有，檢查系統是否有安裝slack，如果有，回到主選單。如果沒有，幫使用者安裝。安裝完打開slack app(安裝指令：`brew install --cask slack`)
8. raycast，檢查系統是否有安裝homebrew，如果沒有顯示訊息「需要先安裝homebrew」並且回到主選單，如果有，檢查系統是否有安裝raycast，如果有，回到主選單。如果沒有，幫使用者安裝。安裝完打開raycast app(安裝指令：`brew install --cask raycast`)

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
   4. 安裝完之後可以幫使用者下載最新的lts nodejs
   5. 檢查nodejs使否是最新的
   6. 檢查nodejs使否是lts
4. vscode
   1. 可以正確辨識是否有安裝homebrew
   2. 可以正確安裝
   3. 可以正確顯示版本
5. google chrome
   1. 可以正確辨識是否有安裝homebrew
   2. 可以正確安裝
   3. 可以正確顯示版本
6. docker
   1. 可以正確辨識是否有安裝homebrew
   2. 可以正確安裝
   3. 可以正確顯示版本
7. slack
   1. 可以正確辨識是否有安裝homebrew
   2. 可以正確安裝
   3. 可以正確顯示版本
8. raycast
   1. 可以正確辨識是否有安裝homebrew
   2. 可以正確安裝
   3. 可以正確顯示版本