package main

import "core:fmt"
import "core:log"
import "core:os"
import "core:strconv"
import "core:text/regex"


part1 :: proc(data: []u8) -> i32 {
	text := string(data)
	pat, err := regex.create("mul\\((\\d+),(\\d+)\\)", {.Global})
	if err != nil do fmt.panicf("Couldn't create regex pattern:", err)
	defer regex.destroy(pat)

	slice := text[:]
	total: i32
	for {
		cap, ok := regex.match(pat, slice)
		if !ok do break
		defer {
			slice = slice[cap.pos[0][1]:]
			regex.destroy(cap)
		}

		f1 := i32(strconv.atoi(cap.groups[1]))
		f2 := i32(strconv.atoi(cap.groups[2]))
		p := f1 * f2
		total += p
	}

	return total
}

part2 :: proc(data: []u8) -> i32 {
	text := string(data)

	pat, err1 := regex.create("(?:mul\\((\\d+),(\\d+)\\))|(?:do\\(\\))|(?:don't\\(\\))", {.Global})
	if err1 != nil do fmt.panicf("Couldn't create regex pattern:", err1)
	defer regex.destroy(pat)

	slice := text[:]
	enabled := true
	total: i32
	for {
		cap, ok := regex.match(pat, slice)
		if !ok do break
		defer {
			slice = slice[cap.pos[0][1]:]
			regex.destroy(cap)
		}

		if cap.groups[0][:3] == "mul" {
			if enabled {
				f1 := i32(strconv.atoi(cap.groups[1]))
				f2 := i32(strconv.atoi(cap.groups[2]))
				p := f1 * f2
				total += p
			}
		} else if cap.groups[0] == "do()" {
			enabled = true
		} else if cap.groups[0] == "don't()" {
			enabled = false
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
	data, ok := os.read_entire_file("ex1.txt")
	if !ok do return
	defer delete(data)
	testing.expect(t, part1(data) == 161)
}

@(test)
test_part2 :: proc(t: ^testing.T) {
	data, ok := os.read_entire_file("ex2.txt")
	if !ok do return
	defer delete(data)
	testing.expect(t, part2(data) == 48)
}
