package main

import (
	"bufio"
	"fmt"
	"os"
	"strings"
	"time"
)

type data struct {
	direction                string
	initialGrid              map[string]string
	grid                     map[string]string
	initialCharacterPosition string
	characterPosition        string
    characterMove            int
	iter                     int

	obstacleIndex int

	loopPosition  string
	loopDirection string

	countLoop int
	countAll  int
}

const (
	empty     = "."
	occupied  = "#"
	visited   = "X"
	character = "^"
	obstacle  = "O"
)

func (d *data) prepare(filename string) {
	file, err := os.Open(filename)
	if err != nil {
		panic(fmt.Errorf("error opening file: %w", err))
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)

	row := 0
	d.initialGrid = make(map[string]string)
	d.grid = make(map[string]string)
	for scanner.Scan() {
		line := scanner.Text()
		chars := strings.Split(line, "")
		for col, char := range chars {
			position := "x" + fmt.Sprintf("%d", col) + "y" + fmt.Sprintf("%d", row)
			d.initialGrid[position] = char
			d.grid[position] = char
		}
		row++
	}
}

func (d *data) findCharacter() {
	for k, v := range d.grid {
		if v == character {
			d.characterPosition = k
			d.initialCharacterPosition = k
			return
		}
	}
	panic("Character not found")
}

func (d *data) moveCharacter(log bool, withObstacle bool) {
	d.grid[d.characterPosition] = visited
	initPosition := d.characterPosition
	initDirection := d.direction

	nextPosition := getNextPosition(d.characterPosition, d.direction)
	if d.grid[nextPosition] == "" {
		if log {
			d.log()
		}
		return
	}
	for d.grid[nextPosition] == occupied || d.grid[nextPosition] == obstacle {
		d.direction = turnRight(d.direction)
		nextPosition = getNextPosition(d.characterPosition, d.direction)
	}
	if d.iter == d.obstacleIndex && withObstacle {
		d.grid[nextPosition] = obstacle
		d.direction = turnRight(d.direction)
		d.loopPosition = initPosition
		d.loopDirection = initDirection
		nextPosition = getNextPosition(d.characterPosition, d.direction)
	}
	for d.grid[nextPosition] == occupied || d.grid[nextPosition] == obstacle {
		d.direction = turnRight(d.direction)
		nextPosition = getNextPosition(d.characterPosition, d.direction)
	}

	if initPosition == d.loopPosition && initDirection == d.loopDirection {
		d.countLoop++
	}
	if d.countLoop > 2 {
		d.countAll++
		return
	}

	d.grid[nextPosition] = character
	d.characterPosition = nextPosition
	if log {
		d.log()
	}
	d.iter++
    d.characterMove++

	d.moveCharacter(log, withObstacle)
}

func (d *data) moveCharacterWithObstacle(log bool) {
	d.moveCharacter(false, false)
    moves := d.characterMove
    fmt.Println("Moves:", moves)
	for moves > 0 {
		d.iter = 0
		d.countLoop = 0
		d.grid = make(map[string]string)
		d.characterPosition = d.initialCharacterPosition
		d.direction = "N"
		for k, v := range d.initialGrid {
			d.grid[k] = v
		}
		d.moveCharacter(log, true)
		d.obstacleIndex++
        moves--
	}
}

func (d *data) countVisited() int {
	count := 0
	for _, v := range d.grid {
		if v == visited {
			count++
		}
	}
	return count
}

func (d *data) log() {
	array := gridToArray(d.grid)
	for _, v := range array {
		fmt.Println(v)
	}
	fmt.Println("------------------------------------------------")
	time.Sleep(50 * time.Millisecond)
}

func turnRight(direction string) string {
	switch direction {
	case "N":
		return "E"
	case "E":
		return "S"
	case "S":
		return "W"
	case "W":
		return "N"
	}
	return ""
}

func getNextPosition(position string, direction string) string {
	x, y := parsePosition(position)
	switch direction {
	case "N":
		y--
	case "E":
		x++
	case "S":
		y++
	case "W":
		x--
	}
	return "x" + fmt.Sprintf("%d", x) + "y" + fmt.Sprintf("%d", y)
}

func parsePosition(position string) (int, int) {
	x, y := 0, 0
	_, err := fmt.Sscanf(position, "x%dy%d", &x, &y)
	if err != nil {
		panic(fmt.Errorf("error parsing position: %w", err))
	}
	return x, y
}

func gridToArray(grid map[string]string) [][]string {
	var array [][]string
	maxX, maxY := 0, 0
	for k := range grid {
		x, y := parsePosition(k)
		if x > maxX {
			maxX = x
		}
		if y > maxY {
			maxY = y
		}
	}

	for y := 0; y <= maxY; y++ {
		var row []string
		for x := 0; x <= maxX; x++ {
			position := "x" + fmt.Sprintf("%d", x) + "y" + fmt.Sprintf("%d", y)
			row = append(row, grid[position])
		}
		array = append(array, row)
	}
	return array

}

func run_part_1(filename string) {
	data := data{
		direction: "N",
	}
	data.prepare(filename)
	data.findCharacter()
	data.moveCharacter(false, false)
    fmt.Println(data.countVisited())
}

func run_part_2(filename string) {
	data := data{
		direction: "N",
	}
	data.prepare(filename)
	data.findCharacter()
	data.moveCharacterWithObstacle(false)
	fmt.Println(data.countAll)
}

func main() {
	run_part_1("input_test.txt")
    run_part_1("input.txt")
	run_part_2("input_test.txt")
    run_part_2("input.txt")
}
