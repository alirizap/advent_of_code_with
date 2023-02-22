package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strconv"
	"strings"
)

func parseInputFile(filename string) ([]string, error) {
	file, err := os.Open(filename)
	if err != nil {
		return nil, err
	}
	defer file.Close()

	lines := []string{}
	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		lines = append(lines, scanner.Text())
	}

	if err := scanner.Err(); err != nil {
		return nil, err
	}

	return lines, nil
}

func performInstruction(inst string, a, b, cur *int) {

	switch {
	case strings.Contains(inst, "tpl"):
		if strings.Contains(inst, "a") {
			*a *= 3
		} else {
			*b *= 3
		}
		*cur += 1
		return
	case strings.Contains(inst, "hlf"):
		if strings.Contains(inst, "a") {
			*a /= 2
		} else {
			*b /= 2
		}
		*cur += 1
		return
	case strings.Contains(inst, "inc"):
		if strings.Contains(inst, "a") {
			*a += 1
		} else {
			*b += 1
		}
		*cur += 1
		return
	case strings.Contains(inst, "jmp"):
		_, after, _ := strings.Cut(inst, " ")
		offset, _ := strconv.Atoi(after)
		*cur += offset
		return
	case strings.Contains(inst, "jie"):
		_, after, _ := strings.Cut(inst, ", ")
		offset, _ := strconv.Atoi(after)
		var isEven bool
		if strings.Contains(inst, "a") {
			isEven = *a%2 == 0
		} else {
			isEven = *b%2 == 0
		}

		if isEven {
			*cur += offset
			return
		}
	case strings.Contains(inst, "jio"):
		_, after, _ := strings.Cut(inst, ", ")
		offset, _ := strconv.Atoi(after)
		var isEven bool
		if strings.Contains(inst, "a") {
			isEven = *a == 1
		} else {
			isEven = *b == 1
		}

		if isEven {
			*cur += offset
			return
		}
	}
	*cur += 1
}

func run(lines []string) {
	a := 0
	// a := 1 uncomment for part two
	b := 0

	cur := 0
	for cur < len(lines) {
		performInstruction(lines[cur], &a, &b, &cur)
	}

	fmt.Println("b: ", b)
}

func main() {
	lines, err := parseInputFile("23")
	if err != nil {
		log.Fatal(err)
	}

	run(lines)
}
