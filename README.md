# Dragonblaze

Dragonblaze is a tool to use RGB led strips in your stop motion projects. It is a proxy that controls a [Pixelblaze](https://www.bhencke.com/pixelblaze) using DMX and frame position information from [Dragonframe](https://www.dragonframe.com).

You can create animations in Pixelblaze. Use lightning channels in Dragonframe which are mapped to variables on the Pixelblaze side. This lets you create animated lightning effects that look fluid when shot in stop motion:

![LED example](docs/example.gif)

## How to use

Some example Pixelblaze code:

```javascript
export var position = 0
export var framesPerSecond = 12
export var useStopmotionTime = false

function stopmotionTime(interval) {
  if (useStopmotionTime) {
    return mod(position / framesPerSecond / 65.536 / interval , 1)
  } else {
    return time(interval)
  }
}

export function beforeRender(delta) {
  t1 = stopmotionTime(0.1)
}

export function render(index) {
  h = t1 + index / pixelCount
  s = 1
  v = 1
  hsv(h, s, v)
}
```

## Setup

There are some setup steps required to run this. We hope to make this easier to get done in the future.

### Rust

Dragonblaze is written in Rust. Currently you need to [install Rust](https://forge.rust-lang.org/infra/other-installation-methods.html#other-ways-to-install-rustup). Checkout this project. Then run the following command in the terminal to install:

```
cargo build --release
```

### Pixelblaze

Setup your Pixelblaze and fetch its IP address. You can then start dragonblaze in the terminal:

```
target/release/dragonblaze <ip>
```

It will log what it is doing and if the connection is successful.

### Dragonframe

On the Dragonframe end you have to setup a script and Artnet
connnection.

#### Script

Open Dragonframe preferences. Then go to Advanced. Check "Enable action script". In the input below put `<project location>/target/release/dragonframe`.

#### Artnet connection

Open your scene. Then open Scene > Connections. Create an "ArtNet DMX" connection and enter `127.0.0.1`. Everything should now be working.
