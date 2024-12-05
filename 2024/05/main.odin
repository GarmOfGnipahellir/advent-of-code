package main

import "core:fmt"
import "core:log"
import "core:os"
import "core:slice"
import "core:strconv"
import "core:strings"

Depends :: map[int][dynamic]int

validate_order :: proc(nums: []int, depends: Depends) -> bool {
	for num, i in nums {
		after := nums[i + 1:]

		deps, ok := depends[num]
		for dep in deps {
			if slice.contains(after, dep) {
				return false
			}
		}
	}
	return true
}

part1 :: proc(data: []u8) -> int {
	text := string(data)
	ok: bool
	text, ok = strings.remove_all(text, "\r")

	offs := strings.index(text, "\n\n")
	depends_text := text[:offs]
	order_text := text[offs + 2:]

	depends := make(Depends)
	defer {
		for k, v in depends {
			delete(v)
		}
		delete(depends)
	}
	for line in strings.split_lines_iterator(&depends_text) {
		offs = strings.index(line, "|")
		before := strconv.atoi(line[:offs])
		after := strconv.atoi(line[offs + 1:])

		if after not_in depends {
			depends[after] = make([dynamic]int, 0, 1)
		}
		append(&depends[after], before)
	}

	total: int
	for line in strings.split_lines_iterator(&order_text) {
		line := line

		nums: [dynamic]int
		defer delete(nums)
		for num_str in strings.split_iterator(&line, ",") {
			num := strconv.atoi(num_str)
			append(&nums, num)
		}

		is_valid := validate_order(nums[:], depends)
		if !is_valid do continue

		middle := len(nums) / 2
		total += nums[middle]
	}

	return total
}

part2 :: proc(data: []u8) -> int {
	text := string(data)
	ok: bool
	text, ok = strings.remove_all(text, "\r")

	offs := strings.index(text, "\n\n")
	depends_text := text[:offs]
	order_text := text[offs + 2:]

	depends := make(Depends)
	defer {
		for k, v in depends {
			delete(v)
		}
		delete(depends)
	}
	for line in strings.split_lines_iterator(&depends_text) {
		offs = strings.index(line, "|")
		before := strconv.atoi(line[:offs])
		after := strconv.atoi(line[offs + 1:])

		if after not_in depends {
			depends[after] = make([dynamic]int, 0, 1)
		}
		append(&depends[after], before)
	}

	total: int
	for line in strings.split_lines_iterator(&order_text) {
		line := line

		nums: [dynamic]int
		defer delete(nums)
		for num_str in strings.split_iterator(&line, ",") {
			num := strconv.atoi(num_str)
			append(&nums, num)
		}

		is_valid := validate_order(nums[:], depends)
		if is_valid do continue

		log.info(nums)
		for !validate_order(nums[:], depends) {
			log.info(" ", nums)
			for i in 0 ..< len(nums) {
				log.info("   ", i, nums[i])
				for j in i + 1 ..< len(nums) {
					log.info("     ", j, nums[j])
                    if slice.contains(depends[nums[i]][:], nums[j]) {
                        nums[i], nums[j] = nums[j], nums[i]
                    }
				}
			}
		}

		middle := len(nums) / 2
		total += nums[middle]
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
	testing.expect_value(t, part1(data), 143)
}

@(test)
test_part2 :: proc(t: ^testing.T) {
	data, ok := os.read_entire_file("ex.txt")
	if !ok do return
	defer delete(data)
	testing.expect_value(t, part2(data), 123)
}
