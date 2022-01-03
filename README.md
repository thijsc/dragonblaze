# Dragonblaze

Dragonblaze is a tool to use RGB led strips in your stop motion projects. It is a proxy that controls a [Pixelblaze](https://www.bhencke.com/pixelblaze) using lightning and frame position information from [Dragonframe](https://www.dragonframe.com).

## Setup

There are some setup steps required to run this. We hope to make this easier to get done in the future.

### Rust

Dragonblaze is written in Rust. Currently you need to [install Rust](https://forge.rust-lang.org/infra/other-installation-methods.html#other-ways-to-install-rustup). Then run the following command in the terminal to install:

```
cargo install dragonblaze
```

### Pixelblaze

Setup your Pixelblaze and fetch its IP address. You can then start dragonblaze in the terminal:

```
dragonblaze <ip>
```

It will log what it is doing and if the connection is successful.

### Dragonframe

On the Dragonframe end you have to setup a script and Artnet
connnection.

#### Script

Open Dragonframe preferences. Then go to Advanced. Check "Enable action script". In the input below put `~/.cargo/bin/dragonframe`.

#### Artnet connection

Open your scene. Then open Scene > Connections. Create an "ArtNet DMX" connection and enter `127.0.0.1`.
