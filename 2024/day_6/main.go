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
}

var obstaclesPositions []string

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
	for k, v := range d.grid {
		if v == character {
			d.characterPosition = k
			d.initialCharacterPosition = k
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
	if grid[position] == empty || grid[position] == visited {
		return position, direction
	}
	nextDirection := turnRight(direction)
	return findNextPossiblePosition(grid, currentPosition, nextDirection)
}

func moveCharacter(grid map[string]string, characterPosition string, direction string, withObstacle bool, obstaclePosition string) {
	visitedPositions := make(map[string]int)
	for {
		visitedPositions[characterPosition]++
		grid[characterPosition] = visited
		nextPosition, newDirection := findNextPossiblePosition(grid, characterPosition, direction)
		if nextPosition == "" {
			break
		}
		if withObstacle {
			newGrid := make(map[string]string)
			for k, v := range grid {
				newGrid[k] = v
			}
			newGrid[nextPosition] = obstacle
	//		p, d := findNextPossiblePosition(newGrid, characterPosition, direction)
			moveCharacter(newGrid, characterPosition, direction, false, nextPosition)
		}
		characterPosition = nextPosition
		direction = newDirection
		if visitedPositions[characterPosition] > 5 && !withObstacle {
			obstaclesPositions = append(obstaclesPositions, obstaclePosition)
			break
		}
	}
}

func run_part_1(filename string) {
	data := data{
		direction: "N",
	}
	data.prepare(filename)
	moveCharacter(data.grid, data.characterPosition, data.direction, false, "")
	// Count the number of visited positions
	count := 0
	for _, v := range data.grid {
		if v == visited {
			count++
		}
	}
	fmt.Println("Number of visited positions:", count)
}

func run_part_2(filename string) {
	data := data{
		direction: "N",
	}
	data.prepare(filename)
    initialPosition := data.characterPosition
	moveCharacter(data.grid, data.characterPosition, data.direction, true, "")
    // Filter unique obstacles
    obstacles := make(map[string]bool)
    for _, v := range obstaclesPositions {
        if v == initialPosition {
            continue
        }
        obstacles[v] = true
    }
    fmt.Println("Number of unique obstacles:", len(obstacles))
}

func main() {
	// run_part_1("input_test.txt")
	// run_part_2("input_test.txt")
	run_part_2("input.txt")
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
	time.Sleep(50 * time.Millisecond)
}
