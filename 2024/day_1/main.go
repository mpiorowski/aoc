package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"sort"
	"strconv"
	"strings"
)

func processFile(filename string) ([]int, []int, error) {
	file, err := os.Open(filename)
	if err != nil {
		return nil, nil, fmt.Errorf("error opening file: %w", err)
	}
	defer file.Close()

	var leftNumbers, rightNumbers []int
	scanner := bufio.NewScanner(file)

	for scanner.Scan() {
		line := strings.TrimSpace(scanner.Text())
		if line == "" {
			continue // Skip empty lines
		}

		// Remove all spaces from the line
		line = strings.ReplaceAll(line, " ", "")

		// Validate line length is even
		if len(line)%2 != 0 {
			return nil, nil, fmt.Errorf("invalid line format: %s", line)
		}

		midpoint := len(line) / 2
		leftStr, rightStr := line[:midpoint], line[midpoint:]

		leftNum, err := strconv.Atoi(leftStr)
		if err != nil {
			return nil, nil, fmt.Errorf("invalid left number: %s, error: %w", leftStr, err)
		}

		rightNum, err := strconv.Atoi(rightStr)
		if err != nil {
			return nil, nil, fmt.Errorf("invalid right number: %s, error: %w", rightStr, err)
		}

		leftNumbers = append(leftNumbers, leftNum)
		rightNumbers = append(rightNumbers, rightNum)
	}

	if err := scanner.Err(); err != nil {
		return nil, nil, fmt.Errorf("error reading file: %w", err)
	}

	return leftNumbers, rightNumbers, nil
}

func calculateDifference(left, right []int) int {
	if len(left) != len(right) {
		log.Fatal("Mismatched slice lengths")
	}

	sum := 0
	for i := range left {
		sum += abs(left[i] - right[i])
	}
	return sum
}

// calculateOccurrenceSum calculates the sum based on occurrences
func calculateOccurrenceSum(left, right []int) int {
	// Use a more efficient method for counting occurrences
	occurrenceMap := make(map[int]int)
	for _, num := range right {
		occurrenceMap[num]++
	}

	sum := 0
	for _, num := range left {
		sum += num * occurrenceMap[num]
	}
	return sum
}

func abs(x int) int {
	if x < 0 {
		return -x
	}
	return x
}

func main() {
	filename := "input.txt"
	leftNumbers, rightNumbers, err := processFile(filename)
	if err != nil {
		log.Fatalf("Error processing file: %v", err)
	}

	sort.Ints(leftNumbers)
	sort.Ints(rightNumbers)

	differenceSum := calculateDifference(leftNumbers, rightNumbers)
	fmt.Println(differenceSum)

	occurrenceSum := calculateOccurrenceSum(leftNumbers, rightNumbers)
	fmt.Println(occurrenceSum)
}
