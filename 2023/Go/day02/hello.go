package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strconv"
	"strings"
)

func main() {
	games := parseInput("../../inputs/day02/input.txt")
	fmt.Printf("part 1: %d\n", part1(games))
	fmt.Printf("part 2: %d\n", part2(games))
}

func part1(games [][]Cubes) (sum uint) {
	for i, game := range games {
		min_cubes := min_cubes(game)
		if min_cubes.red <= 12 && min_cubes.green <= 13 && min_cubes.blue <= 14 {
			sum += uint(i) + 1
		}
	}
	return
}

func part2(games [][]Cubes) (sum uint) {
	for _, game := range games {
		min_cubes := min_cubes(game)
		sum += min_cubes.power()
	}
	return
}

type Cubes struct {
	red   uint
	green uint
	blue  uint
}

func (c Cubes) power() uint {
	return c.red * c.green * c.blue
}

func parseInput(path string) (games [][]Cubes) {
	file, err := os.Open(path)
	if err != nil {
		log.Fatal(err)
		return
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)

	for scanner.Scan() {
		line := strings.Trim(strings.Split(scanner.Text(), ":")[1], " ")
		var game []Cubes

		for _, draw := range strings.Split(line, ";") {
			var red uint = 0
			var green uint = 0
			var blue uint = 0
			for _, color := range strings.Split(strings.Trim(draw, " "), ", ") {
				split := strings.Split(color, " ")
				num, _ := strconv.Atoi(split[0])
				switch split[1] {
				case "red":
					red = uint(num)
				case "green":
					green = uint(num)
				case "blue":
					blue = uint(num)
				}
			}
			game = append(game, Cubes{red, green, blue})
		}
		games = append(games, game)
	}

	if err := scanner.Err(); err != nil {
		log.Fatal(err)
	}

	return
}

func min_cubes(game []Cubes) (min_cubes Cubes) {
	min_cubes = Cubes{0, 0, 0}

	for _, draw := range game {
		min_cubes.red = max(min_cubes.red, draw.red)
		min_cubes.green = max(min_cubes.green, draw.green)
		min_cubes.blue = max(min_cubes.blue, draw.blue)
	}

	return
}
