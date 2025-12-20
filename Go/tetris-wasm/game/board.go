package game

//BoardWidth is the number of columns in the Tetris board
const BoardWidth = 10

//BoardHeight is the number of rows in the Tetris board
const BoardHeight = 20

/*Board represents the Tetris playfield
Each cell stores an integer:
  0  -> empty cell
  >0 -> occupied by a locked piece (value corresponds to Piece.ID) */
type Board struct {
	Cells [BoardHeight][BoardWidth]int
}

/*IsEmpty checks whether a given (x, y) position on the board is empty

It returns false if:
  - the coordinates are out of bounds
  - the cell is already occupied
This is used for collision detection when moving or rotating pieces */

func (b *Board) IsEmpty(x, y int) bool {
	/*treat out-of-bounds positions as non-empty
	so pieces cannot move outside the board */
	if x < 0 || x >= BoardWidth || y < 0 || y >= BoardHeight {
		return false
	}

	//A cell is empty if its value is 0
	return b.Cells[y][x] == 0
}

/*Lock permanently places a piece onto the board
It copies each occupied cell of the piece into the board grid,
marking them with the piece's ID. This is called when a piece
can no longer move down */

func (b *Board) Lock(p *Piece) {
	for _, c := range p.Cells() {
		b.Cells[c.Y][c.X] = p.ID
	}
}

