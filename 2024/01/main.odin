package main

import "core:fmt"
import "core:log"
import "core:math"
import "core:os"
import "core:slice"
import "core:strconv"
import "core:strings"

part1 :: proc(input: ^string) -> i32 {
	left, right: [dynamic]i32
	defer delete(left)
	defer delete(right)
	for line in strings.split_lines_iterator(input) {
		splits := strings.split(line, "   ")
		defer delete(splits)
		ln := i32(strconv.atoi(splits[0]))
		rn := i32(strconv.atoi(splits[1]))
		append(&left, ln)
		append(&right, rn)
	}
	slice.sort(left[:])
	slice.sort(right[:])
	total: i32
	for i in 0 ..< len(left) {
		ln := left[i]
		rn := right[i]
		d := abs(ln - rn)
		log.info(ln, "-", rn, ">", d)
		total += d
	}
	return total
}

part2 :: proc(input: ^string) -> i32 {
	left: [dynamic]i32
	right := make(map[i32]i32)
	defer delete(left)
	defer delete(right)
	for line in strings.split_lines_iterator(input) {
		splits := strings.split(line, "   ")
		defer delete(splits)
		ln := i32(strconv.atoi(splits[0]))
		rn := i32(strconv.atoi(splits[1]))
		append(&left, ln)
		elem, ok := right[rn]
		if !ok {
			right[rn] = 1
		} else {
			right[rn] = elem + 1
		}
	}
	total: i32
	for i in 0 ..< len(left) {
		ln := left[i]
		rn := right[ln] or_else 0
		s := ln * rn
		log.info(ln, "*", rn, ">", s)
		total += s
	}
	return total
}

main :: proc() {
	data, ok := os.read_entire_file("in.txt")
	if !ok {
		fmt.panicf("Couldn't read input file.")
	}
	defer delete(data)

	input := string(data)
	fmt.println("Part 1:", part1(&input))
	input = string(data)
	fmt.println("Part 2:", part2(&input))
}

import "core:testing"

@(test)
test_part1 :: proc(t: ^testing.T) {
	data, ok := os.read_entire_file("ex.txt")
	if !ok do return
	defer delete(data)
	input := string(data)
	testing.expect(t, part1(&input) == 11)
}

@(test)
test_part2 :: proc(t: ^testing.T) {
	data, ok := os.read_entire_file("ex.txt")
	if !ok do return
	defer delete(data)
	input := string(data)
	testing.expect(t, part2(&input) == 31)
}
