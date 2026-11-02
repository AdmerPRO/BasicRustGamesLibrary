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

## Requirements

- [Rust](https://www.rust-lang.org/tools/install)
- Cargo (included with Rust)

## Running a Game

Navigate to the directory of the game you want to play:

```bash
cd console/example-game
```

Build and run it:

```bash
cargo run
```

For an optimized release build:

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