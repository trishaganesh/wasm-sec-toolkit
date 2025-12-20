package game

/*Game represents the overall game state.
It holds:
  - the game board (locked pieces)
   - the currently falling active piece */
type Game struct {
	Board  *Board
	Active *Piece
}

/*NewGame initializes a new game instance.

 It creates an empty board and spawns the first active piece */
func NewGame() *Game {
	return &Game{
		Board:  &Board{},
		Active: NewIPiece(),
	}
}

/*Tick advances the game state by one step.

Each tick attempts to move the active piece down by one cell
If the piece collides with the bottom of the board or an
occupied cell, it is locked in place and a new piece is spawned */

func (g *Game) Tick() {
	//then move the active piece down by one row
	g.Active.Pos.Row++

	//then check for collisions after the move
	for _, c := range g.Active.Cells() {
		/*collision occurs if the piece goes out of bounds
		 or overlaps an already occupied cell */
		if c.Row >= BoardHeight || !g.Board.IsEmpty(c.Col, c.Row) {
			//then revert the movement
			g.Active.Pos.Row--

			//lock the piece into the board
			g.Board.Lock(g.Active)

			//then spawn a new falling piece
			g.Active = NewIPiece()
			return
		}
	}
}
