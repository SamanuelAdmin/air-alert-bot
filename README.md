# 🚨 Air Alert Telegram Bot (Ukraine)

Telegram bot for getting notifications about air alerts in ukrainian regions.

## 📌 Description

The bot is designed for fast and simple notifications about air alerts on regions of Ukraine via Telegram.
Developed in Rust, which allow it to start on powerless devices.
It allows to convinient configuration for specific regions, setting an update time, also supports custom message templates.
#### Bot takes data from <a href="https://alerts.in.ua/en">Air Raid Alert map</a>.

## ⚡Opportunities

- 🚨 Notifications about start and finish of alert
- 📍 Regions supports
- 🗎 Message templates
- ⚙  Flexible configuration
- 🤖 Simple integrations with Telegram bot API

## 🛠️ Dependencies

- Teloxide 0.17.0
- Reqwest 0.13.2
- Serde / Tokio / Tera

## 📦 Installation

### 0. Install cargo via your packet manager.
#### For example:
- <code>apt install cargo</code>
- <code>pacman -S cargo</code>

### 1. Clone repository

```bash
git clone https://github.com/SamanuelAdmin/air-alert-bot
cd air-alert-bot
```

### 2. Configure
Create your own <i>.env</i> file using template:
<code>cp .env.template .env</code>
<strong>You also need <a href="https://alerts.in.ua/">your own token</a> to have an opportunity to get data from <a href="https://alerts.in.ua/">https://alerts.in.ua/</a>.</strong>
Then insert your token to the .env file and make changes in config file.

### 3. Build the project
```bash
cargo build --release
```
❗Make sure all config and env files are in one directory.

### ▶️ Start your build

```bash
cd target/release
./air-alert-bot
```


## 🚀 Deploy

You can start bot:

- on VPS
- in Docker (will be in next versions)
- as systemd unit

## 🤝 Pull requests

We dont have any rules for pull-requesting this project. So, if you wanna make some changes - feel free to do so. Aslo, if you find a bug or have a suggestion - you can open an issue and describe it.


## ⚠️ USEFULL
I can not recommend to use this bot as an official informator. <i>Its just a helper, dont entrust your life to him.</i>

## 📄 License
### <a href="https://en.wikipedia.org/wiki/MIT_License">MIT</a>
