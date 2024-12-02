package main

import "core:fmt"
import "core:log"
import "core:os"
import "core:strconv"
import "core:strings"

Direction :: enum {
	None,
	Increasing,
	Decreasing,
}

check_safe :: proc(nums: []i32) -> bool {
	dir: Direction
	for i in 1 ..< len(nums) - 0 {
		n0 := nums[i - 1]
		n1 := nums[i]
		delta := n1 - n0

		log.info(n1, "-", n0, "=", delta)

		if delta == 0 {
			log.info("unsafe, no change")
			return false
		} else if delta > 0 {
			if i == 1 {
				dir = .Increasing
			} else if dir != .Increasing {
				log.info("unsafe, changed dir")
				return false
			}
		} else {
			if i == 1 {
				dir = .Decreasing
			} else if dir != .Decreasing {
				log.info("unsafe, changed dir")
				return false
			}
		}

		if abs(delta) > 3 {
			log.info("unsafe, big diff")
			return false
		}
	}
	return true
}

part1 :: proc(data: []u8) -> i32 {
	text := string(data)
	result: i32
	for line in strings.split_lines_iterator(&text) {
		split := strings.split(line, " ")
		defer delete(split)

		log.info(line)

		nums: [dynamic]i32
		defer delete(nums)
		for part in split {
			append(&nums, i32(strconv.atoi(part)))
		}
		if check_safe(nums[:]) {
			result += 1
		}
	}
	return result
}

part2 :: proc(data: []u8) -> i32 {
	text := string(data)
	result: i32
	for line in strings.split_lines_iterator(&text) {
		split := strings.split(line, " ")
		defer delete(split)

		log.info(line)

		nums: [dynamic]i32
		defer delete(nums)
		for part in split {
			append(&nums, i32(strconv.atoi(part)))
		}

		if check_safe(nums[:]) {
			result += 1
			continue
		}

		for i in 0 ..< len(nums) {
			new_nums: [dynamic]i32
			defer delete(new_nums)
			for j in 0 ..< len(nums) {
				if i == j do continue
				append(&new_nums, nums[j])
			}
			if check_safe(new_nums[:]) {
				result += 1
				break
			}
		}
	}
	return result
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
	testing.expect(t, part1(data) == 2)
	testing.expect(t, part2(data) == 4)
}
