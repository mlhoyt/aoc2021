package main

import (
	"fmt"
	"strconv"
	"strings"

	"github.com/mlhoyt/aoc2021/go/pkg/utils"
)

func main() {
	input, err := utils.LoadInputFile("day13.test.txt")
	if err != nil {
		panic(err)
	}

	points, folds, err := parse(input)
	if err != nil {
		panic(err)
	}

	answer1, err := part1(points, folds)
	if err != nil {
		panic(err)
	}
	fmt.Printf("part1: %d\n", answer1)

	answer2, err := part2(points, folds)
	if err != nil {
		panic(err)
	}
	fmt.Printf("part2: %d\n", answer2)
}

// Point defines a two-dimensional position
type Point struct {
	X uint
	Y uint
}

// Points is a collection of Point instances
type Points map[Point]struct{}

// Axes defines an enumeration of direction constants
type Axes uint

// Axis enumeration values
const (
	XAxis Axes = iota + 1
	YAxis
)

// Fold defines the axis and position of a fold
type Fold struct {
	Axis  Axes
	Value uint
}

// Folds is a collection of Fold instances
type Folds []Fold

func parse(input string) (Points, Folds, error) {
	points := Points{}
	folds := Folds{}

	mode := "points"
	for _, line := range strings.Split(input, "\n") {
		if line == "" {
			mode = "folds"
			continue
		}

		switch mode {
		case "points": // number,number
			coords := strings.Split(line, ",")
			xCoord, _ := strconv.Atoi(coords[0])
			yCoord, _ := strconv.Atoi(coords[1])

			points[Point{uint(xCoord), uint(yCoord)}] = struct{}{}
		case "folds": // fold along {x,y}=number
			lexemes := strings.Split(line, " ")
			axisAndValue := strings.Split(lexemes[2], "=")
			axis := axisAndValue[0]
			value, _ := strconv.Atoi(axisAndValue[1])

			switch axis {
			case "x":
				folds = append(folds, Fold{XAxis, uint(value)})
			case "y":
				folds = append(folds, Fold{YAxis, uint(value)})
			}
		}
	}

	return points, folds, nil
}

func doFold(points Points, fold Fold) Points {
	foldedPoints := Points{}

	for point := range points {
		foldedX := fold.Value - (point.X - fold.Value)
		foldedY := fold.Value - (point.Y - fold.Value)

		switch fold.Axis {
		case XAxis:
			if point.X > fold.Value {
				foldedPoints[Point{foldedX, point.Y}] = struct{}{}
			} else {
				foldedPoints[Point{point.X, point.Y}] = struct{}{}
			}
		case YAxis:
			if point.Y > fold.Value {
				foldedPoints[Point{point.X, foldedY}] = struct{}{}
			} else {
				foldedPoints[Point{point.X, point.Y}] = struct{}{}
			}
		}
	}

	return foldedPoints
}

func part1(points Points, folds Folds) (int, error) {
	foldedPoints := doFold(points, folds[0])

	return len(foldedPoints), nil
}

func displayPoints(points Points) {
	var maxX uint = 0
	var maxY uint = 0
	for point := range points {
		if point.X > maxX {
			maxX = point.X
		}
		if point.Y > maxY {
			maxY = point.Y
		}
	}

	grid := [][]string{}
	for y := 0; uint(y) <= maxY; y++ {
		row := []string{}
		for x := 0; uint(x) <= maxX; x++ {
			row = append(row, ".")
		}

		grid = append(grid, row)
	}

	for point := range points {
		grid[point.Y][point.X] = "#"
	}

	for _, row := range grid {
		fmt.Printf("%s\n", strings.Join(row, ""))
	}
}

func part2(points Points, folds Folds) (int, error) {
	foldedPoints := points
	for _, fold := range folds {
		foldedPoints = doFold(foldedPoints, fold)
	}

	displayPoints(foldedPoints)
	return 0, nil
}
