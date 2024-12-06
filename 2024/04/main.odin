package main

import "core:fmt"
import "core:log"
import "core:os"
import "core:slice"
import "core:strconv"
import "core:strings"

part1 :: proc(data: []u8) -> int {
	text := string(data)
	text = strings.trim_space(text)
	lines := strings.split_lines(text)
	defer delete(lines)

	height := len(lines)
	width := len(lines[0])
	diag := height * 2 - 1

	total: int
	buf: [4]rune

	for y := 0; y < height; y += 1 {
		for x := 0; x < width - 3; x += 1 {
			for i := 0; i < 4; i += 1 {
				buf[i] = rune(lines[y][x + i])
			}
			if buf == {'X', 'M', 'A', 'S'} || buf == {'S', 'A', 'M', 'X'} {
				total += 1
			}
		}
	}
	for x := 0; x < width; x += 1 {
		for y := 0; y < height - 3; y += 1 {
			for i := 0; i < 4; i += 1 {
				buf[i] = rune(lines[y + i][x])
			}
			if buf == {'X', 'M', 'A', 'S'} || buf == {'S', 'A', 'M', 'X'} {
				total += 1
			}

		}
	}
	for i := 3; i < diag - 3; i += 1 {
		length := (diag / 2) - abs(i - (diag / 2)) + 1
		x := max(width - i - 1, 0)
		y := abs(min(width - i - 1, 0))
		for j := 0; j < length - 3; j += 1 {
			for k := 0; k < 4; k += 1 {
				buf[k] = rune(lines[y + j + k][x + j + k])
			}
			if buf == {'X', 'M', 'A', 'S'} || buf == {'S', 'A', 'M', 'X'} {
				total += 1
			}
		}
	}
	for i := 3; i < diag - 3; i += 1 {
		length := (diag / 2) - abs(i - (diag / 2)) + 1
		x := min(i, width - 1)
		y := abs(min(width - i - 1, 0))
		for j := 0; j < length - 3; j += 1 {
			for k := 0; k < 4; k += 1 {
				buf[k] = rune(lines[y + j + k][x - j - k])
			}
			if buf == {'X', 'M', 'A', 'S'} || buf == {'S', 'A', 'M', 'X'} {
				total += 1
			}
		}
	}

	return total
}

part2 :: proc(data: []u8) -> int {
	text := string(data)
	text = strings.trim_space(text)
	lines := strings.split_lines(text)
	defer delete(lines)

	height := len(lines)
	width := len(lines[0])

	total: int
	for y := 1; y < height - 1; y += 1 {
		for x := 1; x < width - 1; x += 1 {
			cn := rune(lines[y][x])
			if cn != 'A' do continue

			tl := rune(lines[y - 1][x - 1])
			tr := rune(lines[y - 1][x + 1])
			bl := rune(lines[y + 1][x - 1])
			br := rune(lines[y + 1][x + 1])

			d1 := (tl == 'M' && br == 'S') || (tl == 'S' && br == 'M')
			d2 := (tr == 'M' && bl == 'S') || (tr == 'S' && bl == 'M')

			if d1 && d2 {
				total += 1
			}
		}
	}

	return total
}

main :: proc() {
	data, ok := os.read_entire_file("in.txt")
	if !ok {
		fmt.panicf("Couldn't read input file.")
	}
	defer delete(data)

	fmt.println("Part 1:", part1(data))
	fmt.println("Part 2:", part2(data))
}

import "core:testing"

@(test)
test_part1 :: proc(t: ^testing.T) {
	data, ok := os.read_entire_file("ex.txt")
	if !ok do return
	defer delete(data)
	testing.expect_value(t, part1(data), 18)
}

@(test)
test_part2 :: proc(t: ^testing.T) {
	data, ok := os.read_entire_file("ex.txt")
	if !ok do return
	defer delete(data)
	testing.expect_value(t, part2(data), 9)
}
