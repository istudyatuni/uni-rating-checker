# University rating checker

Telegram bot for notifying about rating changes.

## Running locally

You need rust [installed](https://rustup.rs)

- Get token from [@BotFather](https://t.me/BotFather)
- Create `.env` file and fill (this file is used only at compile time)

```
TG_TOKEN=TOKEN-HERE
```

- Clone this repository
- Run from inside project directory

```sh
cargo r --release
```

## Building for production

You need cross [installed](https://github.com/cross-rs/cross#installation) for building static-linked binary.

Follow steps from "Running locally", then run:

```sh
cross b --release --features prod
```

Alternatively, you can build app on the target machine

```sh
cargo b --release --features prod
```

and take the executable file from the `target/release` folder.
