# Basic Rust Games Library

A collection of simple games written in **Rust**.  
The project contains games of different dimensions and styles, created mainly for learning, experimentation and having fun with Rust.

## About

**Basic Rust Games Library (BRGL)** is an open-source collection of small games built with Rust.

The goal is to explore game development using Rust and experiment with different approaches to creating games — from simple terminal-based projects to 2D, 2.5D, 3D and even 4D concepts.

## Game Categories

Games can belong to different categories:

- **Console Games** – Terminal-based games without graphical interfaces.
- **2D Games** – Traditional two-dimensional games.
- **2.5D Games** – Games combining 2D and 3D elements.
- **3D Games** – Fully three-dimensional games.

## Project Structure

```text
BasicRustGamesLibrary/
├── console/
│   └── ...
├── 2d/
│   └── ...
├── 2.5d/
│   └── ...
├── 3d/
│   └── ...
|   LICENSE
└── README.md
```

Each game is stored in its own directory and should include everything needed to build and run it.

## Available Games

- `console/tic_tac_toe` – two-player tic-tac-toe in the terminal.
- `console/guess_number` – guess a randomly selected number from 1 to 100.

## Requirements

- [Rust](https://www.rust-lang.org/tools/install)
- Cargo (included with Rust)

## Running a Game

The repository is a Cargo workspace, so you can check or build all games from its root:

```bash
cargo check --workspace
cargo build --workspace --release
```

Run a selected game from the repository root:

```bash
cargo run -p tic_tac_toe
cargo run -p guess_number
```

Or enter an individual game directory and run it there:

```bash
cargo run
```

For an optimized release run:

```bash
cargo run --release
```

## Technologies

- **Rust** – Main programming language.
- **Cargo** – Package manager and build system.

Individual games may use additional libraries and frameworks.

## Contributing

Contributions are welcome!

You can contribute by:

- Creating new games.
- Improving existing games.
- Fixing bugs.
- Optimizing performance.
- Experimenting with new game mechanics or rendering techniques.

Keep each game independent and include instructions for building and running it.

## License

This project is open-source. See the [LICENSE](LICENSE) file for licensing information.

---

**Made with Rust. Built for fun and learning.**
