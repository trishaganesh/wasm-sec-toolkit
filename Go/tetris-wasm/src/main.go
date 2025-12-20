package main

import (
	"syscall/js"
	"time"

	"github.com/yourusername/wasm-sec-toolkit/Go/tetris-wasm/game"
)

//cellSize is the size (in pixels) of each Tetris block on the canvas
const cellSize = 30

//the global game state and canvas context
var (
	g   *game.Game
	ctx js.Value //the 2D rendering context of the canvas
)

func main() {
	//we prevent Go program from exiting immediately
	c := make(chan struct{}, 0)

	//we initialize a new Tetris game
	g = game.NewGame()

	//we get the canvas 2D context from the browser
	ctx = js.Global().Get("ctx")

	//then we start the main game loop in a separate goroutine
	go gameLoop()

	//we block forever so Go doesn't exit
	<-c
}

/*gameLoop runs the Tetris game at a fixed interval.

Each tick:
   - moves the active piece down
   - locks the piece if necessary
   - renders the board and active piece */
func gameLoop() {
	ticker := time.NewTicker(500 * time.Millisecond)
	for range ticker.C {
		g.Tick()
		render()
	}
}

//then we render draws the entire game state onto the canvas.
func render() {
	//we clear the canvas
	ctx.Call("clearRect", 0, 0, 300, 600)

	//we draw all locked cells on the board
	for y := 0; y < game.BoardHeight; y++ {
		for x := 0; x < game.BoardWidth; x++ {
			if g.Board.Cells[y][x] != 0 {
				drawCell(x, y)
			}
		}
	}

	//we draw the currently falling active piece
	for _, c := range g.Active.Cells() {
		drawCell(c.X, c.Y)
	}
}

//then drawCell draws a single Tetris block at the given board coordinates.
func drawCell(x, y int) {
	ctx.Set("fillStyle", "cyan") //we color of the block
	ctx.Call("fillRect", x*cellSize, y*cellSize, cellSize, cellSize)
}
