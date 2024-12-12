package main

import (
	"bufio"
	"fmt"
	"os"
	"strings"
)

type data struct {
	lab  [][]string
	grid map[string]string
}

const (
	empty     = "."
	occupied  = "#"
	visited   = "X"
	character = "^"
)

func (d *data) prepare(filename string) {
	file, err := os.Open(filename)
	if err != nil {
		panic(fmt.Errorf("error opening file: %w", err))
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)

	row := 0
	d.lab = make([][]string, 0)
	d.grid = make(map[string]string)
	for scanner.Scan() {
		line := scanner.Text()
		chars := strings.Split(line, "")
		for col, char := range chars {
			position := "x" + fmt.Sprintf("%d", col) + "y" + fmt.Sprintf("%d", row)
			d.grid[position] = char
		}
		row++
		d.lab = append(d.lab, chars)
	}
}

func run(filename string) {
	data := data{}
	data.prepare(filename)

    for _, v := range data.lab {
        fmt.Println(v)
    }
	fmt.Println(data.grid)
}

func main() {
	run("input_test.txt")
	// run("input.txt")
}
