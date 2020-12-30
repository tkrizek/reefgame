# Reef Game Library

*This is currently just an experimental repo for learning and experimenting with Rust.*

An attempt to implement a game engine for board game
[Reef](https://boardgamegeek.com/boardgame/244228/reef). This library might
eventually enable implementation of various game clients or servers.

## Rules

[Official Rules](https://www.ultraboardgames.com/reef/game-rules.php)

## Notation

All game actions/states should have a textual representation.

### Piece Colors

* `r`: red
* `g`: green
* `b`: blue (used instead of violet)
* `y`: yellow

### Player Board

Every player has a 4x4 board. Rows are represented with numbers from `1`
through `4` and columns with letters from `i` through `l`. Similarly to a chess
board, the starting point is at the bottom left. Refer to the following diagram
for the layout.

```
4 i4 j4 k4 l4
3 i3 j3 k3 l3
2 i2 j2 k2 l2
1 i1 j1 k1 l1
  i  j  k  l
```

### Coral Stacks

Every stack has a top color and height (refered to as tier). Its notation is
created by a single letter for the top color and a number for the tier, e.g.:

* `r3`: a 3-piece high stack with red on top
* `y1`: a yellow piece

### Coral Stack Placement

Coral stacks can be placed on the player board. This placement can signal a
player move or a board state. Every coral stack placement consists of the coral
stack notation immediately followed by the player board position.

* `g2l4`: a two-piece high stack with green on top in the upper right position
* `b4j2`: a four-high stack with blue on top in the bottom left center spot
