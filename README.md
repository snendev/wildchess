## WildChess

This is a work-in-progress Rust implementation of
[Fairy Chess](https://en.wikipedia.org/wiki/Fairy_chess) using Bevy.

### Story

Author here: When I started this project, I was simply curious to try to make
Chess games with somewhat "random" pieces. It turns out there is a significant
amount of prior art, including definitions for
[piece notation](https://en.wikipedia.org/wiki/Betza%27s_funny_notation) and
[movement notation](https://en.wikipedia.org/wiki/Fairy_chess_piece#Parlett's_movement_notation),
which are even
[implemented in GNU's XBoard](https://www.gnu.org/software/xboard/Betza.html).

Now I am to treat this as a specification and build something similarly-powerful
to experiment with Bevy's flexibility.

### Chess-Like Games

Chess-like games share a number of properties:

- They are turn-based games between some number of players
- They take place on a square grid of tiles (the geometry of the "board"
  [can change](https://en.wikipedia.org/wiki/Fairy_chess#Types_of_fairy_chess_problems))
- Pieces move based on geometric patterns on that grid
- Pieces can capture (i.e. remove from the board) other pieces based on these
  movement patterns
- Some pieces can be "royal", making it unable to be captured without losing the
  game (a generalization of "king" pieces across variants)
- Clocks can be introduced to limit thinking time and prevent stalling

#### Turns

Players alternate turns by moving pieces. In some games, players are allowed to
make multiple moves per turn. Turns can be restricted in special ways, such that
e.g. in DuckChess, some subset of pieces are moved during one phase of the turn,
and a second set of pieces are moved otherwise.

#### Board

Boards are most often square grids, but size and shape can vary. For example,
Chess is 8x8, Shogi is 9x9; additionally, Chess can be played where the `a` and
`h` files "wrap" around. Boards can also be hexagonal or other regular shapes.
For now, this will only consider orthogonal layouts, but exploring more is a
future goal.

#### Pieces

##### Player Control

Pieces can be controlled by a specific player or players. They can also be
neutral, uncontrollable pieces, or neutral, mutually-controllable pieces.

TODO: Implement Doubles chess, where you alternate like table tennis and default
clock is bullet.

(TODO: `GameSetting` component)

TODO: Some kind of Racing Kings with "hurdles"?

##### Movement

Pieces move based on pre-defined geometric patterns. These patterns are defined
by "steps" on the board, some vector between squares executed per step. A
pattern may search the board space via a limited or unlimited number of "steps"
in the same direction.

This search space can behave differently based on "colliding" pieces which are
positioned on the target square of some step. A movement may be blocked before a
collision ("walking") or they may _require_ a collision and only permit movement
to following steps ("hopping"). In either of these modes, pieces may be
permitted to target the colliding square itself, usually resulting in a capture.

Pieces may execute some "series" (or "chain") of patterns. These patterns may be
exhaustive (the entire series must be performed) or non-exhaustive (any subset
of the patterns may be performed).

Pieces also have an "orientation" which orients a piece's move rules along a
particular axis. For example, a piece that can only move "forward" in a
traditional Chess game moves differently for the black pieces than for the white
pieces.

##### Capture

Many pieces can capture another piece when moving. This can occur through a
number of conditions:

- by moving to a collision square ("displacement capture")
- by hopping over a piece ("overtake capture")
- by surrounding (covering on some number of sides) an enemy piece ("custodial
  capture")
- by "shooting" ("range capture")
- by capturing "in passing" (if the last turn was moving the captured piece
  through the target square)

Capture can be restricted only to certain kinds of enemy pieces.

##### Promotion

Pieces can _promote_ under various conditions, usually by reaching somewhere
specific, such as the farthest `Rank` in a position.

In WildChess, promotion is encoded by `Mutation`, which is named to account for
negative-impact promotion variants.

##### Special Behaviors

Many special behaviors can apply to pieces. For example, pieces can be "iron",
preventing capture entirely. Pieces may
"[relay](https://en.wikipedia.org/wiki/Knight_relay_chess)" their patterns,
giving friendly defended pieces additional movement abilities. The list goes on.

Pieces can also have various restrictions placed on their actions. They may only
be able to perform certain patterns on or off specific board squares (such as
specific ranks) or when
[attacked by an enemy piece of the same kind](https://en.wikipedia.org/wiki/Madrasi_chess),
for example.

#### Win Condition

In most chess games, the goal is to capture one or all enemy Royal pieces.
[Checkmate](https://en.wikipedia.org/wiki/Checkmate) is a common substitution.

In variants like [Racing Kings Chess](https://lichess.org/variant/racingKings),
the goal is to reach the 8th rank with the Royal piece.
[King of the Hill Chess](https://lichess.org/variant/kingOfTheHill) is a race to
the center of the board.

These win conditions interpret Royalty in various ways. In King of the Hill
Chess, checkmate is also a win condition. In Racing Kings Chess, any moves that
attack the enemy king are impossible. Games such as
[Sternhalma](https://en.wikipedia.org/wiki/Sternhalma) could be considered a
game of star-shaped hexagonal chess given the correct piece specification, if
all pieces were also Royal under a racing-type win condition. (It would be cool
to implement this much, but that might take a while.)

### Implementation

#### Pieces

#### Board / rule configurations

### Upcoming TODOs

- Castling
- Promotion icon changes
- Gameover UI
- Racing Kings Win Condition
- Integrate Orientation more into the logic (will help extend to 4-player)
- Behavior upgrades: Capture-chain (continue with pattern on capture) and
  composed behaviors (e.g. pattern followed by orthogonal pattern)
- Parameterizable wild configuration
- Wild clock presets
