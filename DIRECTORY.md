# Code Directory

This document briefly explains the layout of the repository and defines the roles of each crate.

Core crates:

- **gameboard**: Defines the traits `GameBoard` and `GameVector` which provide a baseline for defining a grid-like board.
- **chess**: The rules of fairychess implemented using `GameBoard` and `GameVector`.

Implementation crates:

  - TODO: probably condense these layers using cfg or the bevy_game_template fork's structure.
- **checkerboard**: A `GameBoard` implementation creating a basic checkerboard such as for Chess or Shogi.
- **layouts**: A set of `Checkerboard`s that define specific chess variants.
  - TODO: any reason to split these up?

Application crates:

- **app**: Code for the various application builds of the game. For example, wasm requires some specific boilerplate, so this crate provides that layer.
- **ui**: An egui implementation of a chess ui. Has only some basic features.
