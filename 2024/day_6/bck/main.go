package main

import (
	"bufio"
	"flag"
	"fmt"
	"io"
	"log"
	"os"
	"path/filepath"
)

type coordinate struct {
	x      int
	y      int
	facing direction
}

type direction struct {
	dx int
	dy int
}

func (c coordinate) String() string {
	return fmt.Sprintf("(%d, %d)", c.x, c.y)
}

type input struct {
	mat [][]int

	width  int
	height int

	start coordinate
}

func parseInput(src io.Reader) *input {
	i := input{
		mat: make([][]int, 0),
	}

	sc := bufio.NewScanner(src)

	vi := 0

	for sc.Scan() {
		line := sc.Bytes()
		if len(line) == 0 {
			break
		}

		if i.width == 0 {
			i.width = len(line)
		}

		i.mat = append(i.mat, make([]int, i.width))

		for it := 0; it < i.width; it += 1 {
			ch := line[it]

			hasObstable := 0
			switch ch {
			case '.':
				hasObstable = 0
			case '#':
				hasObstable = 1
			case '^':
				i.start = coordinate{x: it, y: vi}
			}

			i.mat[vi][it] = hasObstable
		}

		vi += 1
	}

	i.height = vi

	return &i
}

func printMatrix(inp *input, cur coordinate) {
	fmt.Print("  ")
	for j := 0; j < inp.width; j += 1 {
		fmt.Printf("%d ", j)
	}
	fmt.Println()
	for i := 0; i < inp.height; i += 1 {
		r := inp.mat[i]
		fmt.Printf("%d ", i)

		isStart := cur.y == i

		for j := 0; j < len(r); j += 1 {
			if isStart {
				if cur.x == j {
					fmt.Print("^ ")
					continue
				}
			}
			fmt.Printf("%d ", r[j])
		}
		fmt.Println()
	}
}

func run(in *input) (visited map[int]map[int]int, looped bool) {
	visited = make(map[int]map[int]int, 0)

	exited := func(c coordinate) bool {
		if c.x >= in.width {
			return true
		}
		if c.x < 0 {
			return true
		}
		if c.y >= in.height {
			return true
		}
		if c.y < 0 {
			return true
		}

		return false
	}

	rotate90 := func(c coordinate) coordinate {
		x := 0*c.facing.dx + 1*c.facing.dy
		y := -1*c.facing.dx + 0*c.facing.dy

		c.facing.dx = x
		c.facing.dy = y

		return c
	}

	moveUp := func(c coordinate) coordinate {
		c.x = c.x + c.facing.dx
		c.y = c.y - c.facing.dy

		return c
	}

	moveCur := func(cur coordinate) (altCur coordinate, looped bool) {
		rotatedFor := 0

		for {
			ahead := moveUp(cur)

			if !exited(ahead) {
				front := in.mat[ahead.y][ahead.x]

				if front == 1 {
					if rotatedFor == 4 {
						return cur, true
					}

					cur = rotate90(cur)
					rotatedFor += 1

					continue
				}
			}

			break
		}

		return moveUp(cur), false
	}

	t := in.start
	t.facing = direction{dx: 0, dy: 1}

	for !exited(t) {
		t, looped = moveCur(t)
		if looped {
			return visited, looped
		}

		_, wentTo := visited[t.x]
		if !wentTo {
			visited[t.x] = make(map[int]int)
		}
		if visited[t.x][t.y] > 4 {
			looped = true
			return
		}
		visited[t.x][t.y] += 1
	}

	return
}

func causeLoop(in *input) (count int) {
	for i := 0; i < in.height; i += 1 {
		for j := 0; j < in.width; j += 1 {

			if i == in.start.y {
				if j == in.start.x {
					log.Println("skipped starting position")
					continue
				}
			}

			pos := in.mat[i][j]

			if pos != 0 {
				continue
			}

			in.mat[i][j] = 1

			_, isNowLoop := run(in)
			if isNowLoop {
				count += 1
				log.Printf("obstable in position cause loop: (%d, %d)", j, i)
			}

			in.mat[i][j] = 0
		}
	}

	return
}

func main() {
	inpPtr := flag.String("input", "input.txt", "Path to input file")

	flag.Parse()
	if inpPtr == nil {
		panic("Must provide path to the input file")
	}

	path := *inpPtr
	_, err := os.Stat(path)
	if err != nil {
		log.Fatalf("File not does exists or cannot be stat-ed: %s", err)
	}

	abs, err := filepath.Abs(path)
	if err != nil {
		log.Fatalf("Cannot get absolute file path: %s", err)
	}

	f, err := os.Open(abs)
	if err != nil {
		log.Fatalf("Unable to open file: %s", err)
	}
	defer f.Close()

	parsedInput := parseInput(f)
	printMatrix(parsedInput, parsedInput.start)

	visited, looped := run(parsedInput)
	if !looped {
		count := causeLoop(parsedInput)

		log.Println(count)
	}

	distinct := 0

	for _, r := range visited {
		distinct += len(r)
	}

	log.Println(distinct)
}
