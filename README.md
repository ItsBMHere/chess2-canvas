# About
---


https://user-images.githubusercontent.com/84604651/175720647-803cc54a-555a-4fa5-9e24-7162edb85edf.mp4



`chess2-canvas` is an in-progress board editor for David Sirlin's [Chess 2: The Sequel](https://en.wikipedia.org/wiki/Chess_2:_The_Sequel) and built using the [Bevy game engine](https://bevyengine.org/). It can be used to show off board positions, or play correspondence games (although no rules are implemented).

# Usage
---
At current time of writing (June 24th 2022), all commands are handled via the keyboard. This utility is at a very early stage - the bare minimum functionality is all there, but there's lots of developments to look foward to. 

For now, here are the keyboard commands - if you have ever used [Lichess' board editor](https://lichess.org/editor) then this should feel somewhat familiar:

- **1**: 'Selection' mode - *drag* pieces around by left-clicking and holding; let go to *drop* them on a new square. You can highlight any square on the board in this mode by right-clicking it, but these will disappear as soon as you left-click.
- **2**: Draw King over hovered square
- **3**: Draw Queen
- **4**: Draw Rook
- **5**: Draw Bishop
- **6**: Draw Knight
- **7**: Draw Pawn
- **0**: Delete piece over hovered square.
- **Spacebar**: Toggle between drawing White and Black pieces.
- **A/D**: Cycle through the 6 armies, in the order of: 1) Classic, 2) Nemesis, 3) Empowered, 4) Reaper, 5) Two Kings, 6) Animals. 
I won't explain what all these new pieces do - the [rulebook](https://static1.squarespace.com/static/575f8cb8ab48de461197681a/t/5f2e3acc0fe05d162d2c0834/1596865230951/chess2_rulebook3-0.pdf) does a perfectly good job of introducing them.
- **Ctrl + S**: Save a screenshot of the board configuration to the project directory.



# Problem Decomposition
---

- [x] Draw an 8x8 grid of black and white squares.
- [x] Draw the midline, between ranks 4 and 5.
- [x] Draw notation for files (a-h) and ranks (1-8) 
- [x] Write logic to fill up window with board
- [ ] Write current board state to FEN2 string
- [ ] Flip Board - pieces, and notation
- [x] Save board as PNG (Windows only atm)
- [ ] 'Hand' mode
	- [x] Highlight Piece square when clicked & held
	- [ ] Draw transparent piece on square clicked
	- [x] Draw piece sprite that follows cursor whilst left mouse is held down
	- [x] Draw piece on hovered-over square when left mouse is released
	- [ ] Erase transparent piece from previous square when left mouse released
	- [x] Erase highlight from previous square when left mouse released 
	- [x] Draw circle on hovered-over square when right mouse pressed (sort of? It's more of a square than a circle...)
	- [ ] Draw arrow between right-clicked square and hovered-over square when right mouse pressed, held, & dragged
	- [ ] Erase arrows on left mouse press
	- [x] Erase circles on left mouse press
- [x] 'Pieces' mode
	- [x] Draw Piece on hovered-over square on left mouse press
	- [x] Overwrite piece on hovered-over square on left mouse press with new piece
	- [x] Cycle through different armies with A/D keys
- [x] 'Trash' mode
	- [x] Erase piece on hovered-over square on left mouse press
	(OK - this one is a bit tenuous at the moment...)
- [ ] FEN
	- [ ] Parse FEN2 to board position 
	- [ ] Parse Fischerrandom to board position by value (as per the [most recent ruleset](https://static1.squarespace.com/static/575f8cb8ab48de461197681a/t/5f2e3acc0fe05d162d2c0834/1596865230951/chess2_rulebook3-0.pdf))
