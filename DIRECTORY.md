# Code Directory

This document briefly explains the layout of the repository and defines the roles of each crate.

Core crates:

- **board**: Defines the traits `Board` and `Axes` which provide a baseline for defining a grid-like board.
  - For example, an `Axes` implementation could represent a `Square`, a vertex-based `hexx::Hex`, or a an edge-based `hex::Hex`, and a `Board` implementation could implement some class of board shape.
  - `Axes` can be used by pieces to define properties like movement, vision, etc.
  - (TODO: revisit when `Board` is implemented.)
- **chess**: An implementation of the rules of fairychess generic on `Board` and `Axes`

Implementation crates:

  - TODO: probably condense these layers using cfg or the bevy_game_template fork's structure.
- **checkerboard**: A `Board` implementation creating a basic checkerboard such as for Chess or Shogi.
- **layouts**: A set of `Checkerboard`s that define specific chess variants.
  - TODO: any reason to split these up?

Application crates:

- **app**: Code for the various application builds of the game. For example, wasm requires some specific boilerplate, so this crate provides that layer.
- **ui**: An egui implementation of a chess ui. Has only some basic features.
