package main

import (
	"bufio"
	"fmt"
	"os"
	"slices"
	"strconv"
	"strings"
)

type data struct {
	order     map[int][]int
	inputs    [][]int
	correct   [][]int
	incorrect [][]int
}

func (d *data) prepare(filename string) {
	file, err := os.Open(filename)
	if err != nil {
		panic(fmt.Errorf("error opening file: %w", err))
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)

	isOrder := true
	for scanner.Scan() {
		line := scanner.Text()
		if line == "" {
			isOrder = false
			continue
		}

		if isOrder {
			numbs := strings.Split(line, "|")
			num_1, err := strconv.Atoi(numbs[0])
			if err != nil {
				panic(fmt.Errorf("error converting string to int: %w", err))
			}
			num_2, err := strconv.Atoi(numbs[1])
			if err != nil {
				panic(fmt.Errorf("error converting string to int: %w", err))
			}

			d.order[num_1] = append(d.order[num_1], num_2)
		} else {
			inputs := strings.Split(line, ",")
			line_num := make([]int, 0)
			for _, input := range inputs {
				if input == "" {
					continue
				}
				num, err := strconv.Atoi(input)
				if err != nil {
					panic(fmt.Errorf("error converting string to int: %w", err))
				}
				line_num = append(line_num, num)
			}
			d.inputs = append(d.inputs, line_num)
		}

	}
}

func (d *data) extract_inputs() {
	for _, input := range d.inputs {
		d.check_input(input, d.order)
	}
}

func (d *data) check_input(input []int, order map[int][]int) {
    input, valid := fix_input(input, order)
	if valid {
		d.correct = append(d.correct, input)
	} else {
		d.incorrect = append(d.incorrect, input)
	}
}

func fix_input(input []int, order map[int][]int) ([]int, bool) {
	valid := true
	searched := make([]int, 0)
	for i, num := range input {
		after := order[num]
		for _, s := range searched {
			if slices.Contains(after, s) {
				valid = false
				curr := input[i]
				prev := input[i-1]
				input[i] = prev
				input[i-1] = curr
				input, _ = fix_input(input, order)
				break
			}
		}
		searched = append(searched, num)
	}
	return input, valid
}

func sum_middle(inputs [][]int) int {
	sum := 0
	for _, input := range inputs {
		middleIndex := len(input) / 2
		sum += input[middleIndex]
	}
	return sum
}

func run(filename string) {
	data := data{
		order:  make(map[int][]int),
		inputs: make([][]int, 0),
	}
	data.prepare(filename)
	data.extract_inputs()
	// fmt.Println("Correct inputs:", data.correct)
	// fmt.Println("Incorrect inputs:", data.incorrect)
	fmt.Println("Sum of middle elements of correct inputs:", sum_middle(data.correct))
	fmt.Println("Sum of middle elements of incorrect inputs:", sum_middle(data.incorrect))
}

func main() {
	run("input_test.txt")
	run("input.txt")
}
