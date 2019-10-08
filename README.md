# Rust Tetris

A Tetris clone implemented in Rust, using Piston/OpenGL.

This is my first Rust project, and also my first game. One of the things I was keen to play with was Rust's type system, so I ended up building my logic around a small algebra for the type Cell.
Some key points about the implementation:

1. A cell has a given type, which represents its state, and adding that cell to another produces a potential change in this state.
2. The game board has an inner playing area and a two row/column buffer of cells around it, of type Left, Right, Top or Bottom; whenever a Tetromino cell is added to a Buffer cell, the result will be a Clash state.
3. Each cell of type Tetromino keeps information about its next rotation and rotates individually. A tetromino is a collection of four of these cells, whose rotations are coordinated to produce the patterns for each tetromino.

If you are curious about going through the code, I would recommend starting with cell.rs, then proceeding to board.rs (with a small detour to tetromino.rs), and finally app.rs.

# Playing Instructions

Use the arrows to play (Up to rotate), spacebar to drop tetrominos, and 'p' to pause/unpause.
When the game ends, you can restart it by pressing 'r'.

# License

You are free to use this code as you please.
