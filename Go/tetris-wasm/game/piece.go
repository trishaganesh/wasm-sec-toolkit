package game

/*Vec represents a 2D integer vector
It is used for board coordinates and relative block offsets */
type Vec struct {
	Col, Row int
}

/*Piece represents a falling Tetris piece.
Fields:
  ID     - Unique identifier for the piece type (used for coloring/rendering)
  Blocks - Relative positions of the piece's blocks (local coordinates)
  Pos    - Top-left position of the piece on the board (world coordinates) */
type Piece struct {
	ID     int
	Blocks []Vec
	Pos    Vec
}

/*NewIPiece creates and returns a new "I" tetromino (tetromino is a geometric shape composed of four squares, source: https://en.wikipedia.org/wiki/Tetromino)
The I piece consists of four blocks in a horizontal line
that spawns near the top-center of the board */

func NewIPiece() *Piece {
	return &Piece{
		ID: 1,
		Blocks: []Vec{
			{0, 0}, {1, 0}, {2, 0}, {3, 0},
		},
		Pos: Vec{3, 0},
	}
}

/*Cells returns the absolute board positions occupied by the piece
 It converts each block's local offset into world coordinates by adding the piece's current position
 This is used for:
- collision detection
   - rendering
 - locking the piece into the board */

func (p *Piece) Cells() []Vec {
	cells := make([]Vec, len(p.Blocks))
	for i, b := range p.Blocks {
		cells[i] = Vec{
			Col: p.Pos.Col + b.Col,
			Row: p.Pos.Row + b.Row,
		}
	}
	return cells
}
