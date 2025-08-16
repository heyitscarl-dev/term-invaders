```
     _                            _                     _               
    | |_ ___ _ __ _ __ ___       (_)_ ____   ____ _  __| | ___ _ __ ___ 
    | __/ _ \ '__| '_ ` _ \ _____| | '_ \ \ / / _` |/ _` |/ _ \ '__/ __|
    | ||  __/ |  | | | | | |_____| | | | \ V / (_| | (_| |  __/ |  \__ \
     \__\___|_|  |_| |_| |_|     |_|_| |_|\_/ \__,_|\__,_|\___|_|  |___/

```

A space invaders clone for the terminal.

---

> [!WARNING]
> This project is a work in progress. Features and
> instructions **will** change. 

# Table of Contents

- [Introduction](#introduction)
- [Requirements](#requirements)
- [Installation & Usage](#installation--usage)
- [Controls](#controls)
- [Configuration](#configuration)

# Introduction

**term-invaders** is a retro-inspired clone of the classic _Space Invaders_, built 
entirely for the terminal - from scratch. No game engines, no bulky frameworks - just
pure Rust, [`hecs`](https://github.com/Ralith/hecs), for ECS, and 
[`crossterm`](https://github.com/crossterm-rs/crossterm), for terminal control.

# Roadmap

- [ ] **MVP gameplay**
    - [ ] player movement (horizontal)
    - [ ] player shooting 
    - [ ] first enemy wave
    - [ ] collision detection (bullet vs enemy)
    - [ ] score tracking
    - [ ] game over state
- [ ] **visuals & feedback**
    - [ ] ASCII sprite rendering
    - [ ] basic animations
    - [ ] explosion effects
    - [ ] improved HUD
        - [ ] in-game 
        - [ ] main menu
- [ ] **gameplay enhancements**
    - [ ] multiple enemy types 
    - [ ] increasing difficulty over time 
    - [ ] "power ups"
- [ ] **quality of life**
    - [ ] configurable controls 
    - [ ] pause / resume 
    - [ ] endless mode

# Requirements

- **Rust** (latest stable) - [Install via rustup](https://rustup.rs/)
- A terminal that supports ANSI escape codes (most do)

# Installation & Usage 

Clone the repository and run it directly with Cargo:

```bash
git clone https://github.com/heyitscarl-dev/term-invaders.git 
cd term-invaders 
cargo run --release
```

_or using the GitHub CLI:_

```bash
gh repo clone heyitscarl-dev/term-invaders
cd term-invaders 
cargo run --release
```

When a playable version is released, binaries will be provided for Linux, macOS, and Windows.

# Controls

(Default keybindings - configurable later)

| Key       | Action                                        |
| --------- | --------------------------------------------- |
| `A` / `←` | move left                                     |
| `D` / `→` | move right                                    |
| `Space`   | shoot                                         |
| `Q`       | quit                                          |

# Configuration

Configuration will be handled via a simple `.toml` file in the project root. 

Planned configuration options include:

- Key bindings 
- Screen size / resolution
- Game speed / difficulty scaling 
- Color themes, asset packs

Until configuration is implemented (see [Roadmap](#roadmap)), the game will use sensible defaults.
