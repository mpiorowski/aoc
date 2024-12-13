package main

import (
	"bufio"
	"fmt"
	"os"
	"strings"
	"time"
)

type data struct {
	direction         string
	grid              map[string]string
	initialPosition   string
	characterPosition string
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
	d.grid = make(map[string]string)
	for scanner.Scan() {
		line := scanner.Text()
		chars := strings.Split(line, "")
		for col, char := range chars {
			position := "x" + fmt.Sprintf("%d", col) + "y" + fmt.Sprintf("%d", row)
			d.grid[position] = char
		}
		row++
	}
	for k, v := range d.grid {
		if v == character {
			d.initialPosition = k
			d.characterPosition = k
			return
		}
	}
}

func parsePosition(position string) (int, int) {
	x, y := 0, 0
	_, err := fmt.Sscanf(position, "x%dy%d", &x, &y)
	if err != nil {
		panic(fmt.Errorf("error parsing position: %w", err))
	}
	return x, y
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

func findNextPossiblePosition(
	grid map[string]string,
	currentPosition string,
	direction string,
) (string, string) {
	x, y := parsePosition(currentPosition)
	switch direction {
	case "N":
		y--
	case "S":
		y++
	case "E":
		x++
	case "W":
		x--
	}
	position := "x" + fmt.Sprintf("%d", x) + "y" + fmt.Sprintf("%d", y)
	if grid[position] == "" {
		return "", ""
	}
	if grid[position] == empty || grid[position] == character || grid[position] == visited {
		return position, direction
	}
	nextDirection := turnRight(direction)
	return findNextPossiblePosition(grid, currentPosition, nextDirection)
}

func moveCharacter(
	grid map[string]string,
	characterPosition string,
	direction string,
	visitedPositions map[string]int,
    obstaclePosition string,
	obstacles map[string]bool,
) (map[string]int, map[string]bool) {
	for {
		if characterPosition == "" {
			break
		}
		grid[characterPosition] = visited
		visitedPositions[characterPosition]++
		nextPosition, newDirection := findNextPossiblePosition(grid, characterPosition, direction)
		// Check if we visited any position more then 4 times, then it's a loop
		if visitedPositions[characterPosition] > 4 {
			obstacles[obstaclePosition] = true
			break
		}
		characterPosition = nextPosition
		direction = newDirection
		// log(grid)
	}
	return visitedPositions, obstacles
}

func run(filename string) {
	d := data{
		direction: "N",
	}
	visitedPositions := make(map[string]int)
	obstacles := make(map[string]bool)
	d.prepare(filename)

	newGrid := make(map[string]string)
	for k, v := range d.grid {
		newGrid[k] = v
	}
	moveCharacter(newGrid, d.characterPosition, d.direction, visitedPositions, "", obstacles)

	// Count visited positions
	fmt.Println("Number of visited positions:", len(visitedPositions))

	for v := range visitedPositions {
		newGrid := make(map[string]string)
		for k, v := range d.grid {
			newGrid[k] = v
		}
		// Place obstacle at visited position
		newGrid[v] = obstacle
		d := data{
			direction:         "N",
			characterPosition: d.initialPosition,
		}
        newVisitedPositions := make(map[string]int)
		moveCharacter(newGrid, d.characterPosition, d.direction, newVisitedPositions, v, obstacles)

	}

    // Filter initial position
    delete(obstacles, d.initialPosition)
	fmt.Println("Number of obstacles:", len(obstacles))
}

func main() {
	run("input_test.txt")
	run("input.txt")
}

func log(grid map[string]string) {
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
	for _, v := range array {
		fmt.Println(v)
	}
	fmt.Println("------------------------------------------------")
	time.Sleep(100 * time.Millisecond)
}
