package main

import "core:fmt"
import "core:log"
import "core:os"
import "core:slice"
import "core:strconv"
import "core:strings"
import "core:unicode/utf8"

Direction :: enum {
	North,
	East,
	South,
	West,
}
DirectionVectors :: [Direction][2]int {
	.North = {0, -1},
	.East  = {+1, 0},
	.South = {0, +1},
	.West  = {-1, 0},
}

Level :: struct {
	data:   []rune,
	width:  int,
	height: int,
}

Guard :: struct {
	pos: [2]int,
	dir: Direction,
}

parse_level :: proc(text: string) -> Level {
	lines := strings.split_lines(strings.trim_space(text))
	defer delete(lines)

	level: Level
	level.width = len(lines[0])
	level.height = len(lines)

	level.data = make([]rune, level.width * level.height)
	for y in 0 ..< level.height {
		for x in 0 ..< level.width {
			i := x + y * level.width
			level.data[i] = rune(lines[y][x])
		}
	}

	return level
}

write_level :: proc(level: ^Level) -> string {
	lines := make([]string, level.height)
	for y in 0 ..< level.height {
		start := y * level.width
		end := start + level.width
		lines[y] = utf8.runes_to_string(level.data[start:end])
	}
	text := strings.join(lines, "\n")
	return text
}

is_inside :: proc(level: ^Level, pos: [2]int) -> bool {
	return pos.x >= 0 && pos.x < level.width && pos.y >= 0 && pos.y < level.height
}

turn_right :: proc(dir: Direction) -> Direction {
	return cast(Direction)((cast(int)dir + 1) % 4)
}

rune_at :: proc(level: ^Level, pos: [2]int) -> rune {
	if !is_inside(level, pos) {
		return '.'
	}
	i := pos.x + pos.y * level.width
	return level.data[i]
}

write_snapshots :: proc(
	index: int,
	level: ^Level,
	snaphots1: ^map[[2]int]bit_set[Direction],
	snaphots2: ^map[[2]int]bit_set[Direction],
	obs_pos: [2]int,
) {
	prev_allocator := context.allocator
	context.allocator = context.temp_allocator
	defer context.allocator = prev_allocator

	text: string
	for y in 0 ..< level.height {
		for x in 0 ..< level.width {
			pos: [2]int = {x, y}
			i := x + y * level.width
			dirs1, ok1 := snaphots1[pos]
			dirs2, ok2 := snaphots2[pos]
			dirs := dirs1 + dirs2
			ok := ok1 || ok2
			if ok && pos != obs_pos {
				if rune_at(level, pos) == '^' {
					text = strings.concatenate({text, "^"})
				} else {
					text = strings.concatenate({text, "╬"})
				}
			} else if pos == obs_pos {
				text = strings.concatenate({text, "O"})
			} else {
				rn := rune_at(level, pos)
				if rn == '.' {
					text = strings.concatenate({text, "."})
				} else if rn == '#' {
					text = strings.concatenate({text, "#"})
				} else if rn == '^' {
					text = strings.concatenate({text, "^"})
				}
			}
		}
		text = strings.concatenate({text, "\n"})
	}

	os.write_entire_file(fmt.tprintf("tmp/%04d", index), transmute([]u8)text)
}

part1 :: proc(data: []u8) -> int {
	text := string(data)
	level := parse_level(text)
	defer delete(level.data)

	guard: Guard
	for y in 0 ..< level.height {
		for x in 0 ..< level.width {
			if rune_at(&level, {x, y}) != '^' do continue
			guard.pos = {x, y}
			break
		}
	}

	dir_vecs := DirectionVectors
	visited: [dynamic][2]int
	defer delete(visited)
	for is_inside(&level, guard.pos) {
		next_pos := guard.pos + dir_vecs[guard.dir]

		if rune_at(&level, next_pos) == '#' {
			guard.dir = turn_right(guard.dir)
			continue
		}

		if !slice.contains(visited[:], guard.pos) {
			append(&visited, guard.pos)
		}
		guard.pos = next_pos
	}

	return len(visited)
}

part2 :: proc(data: []u8) -> int {
	text := string(data)
	level := parse_level(text)
	defer delete(level.data)

	guard: Guard
	for y in 0 ..< level.height {
		for x in 0 ..< level.width {
			if rune_at(&level, {x, y}) != '^' do continue
			guard.pos = {x, y}
			break
		}
	}
	start_pos := guard.pos

	dir_vecs := DirectionVectors
	visited: [dynamic][2]int
	defer delete(visited)
	for is_inside(&level, guard.pos) {
		next_pos := guard.pos + dir_vecs[guard.dir]

		if rune_at(&level, next_pos) == '#' {
			guard.dir = turn_right(guard.dir)
			continue
		}

		if !slice.contains(visited[:], guard.pos) {
			append(&visited, guard.pos)
		}
		guard.pos = next_pos
	}

	total: int
	for obs_pos, i in visited {
		if obs_pos == start_pos do continue

		new_level := Level {
			data   = slice.clone(level.data),
			width  = level.width,
			height = level.height,
		}
		defer delete(new_level.data)

		obs_idx := obs_pos.x + obs_pos.y * level.width
		new_level.data[obs_idx] = '#'

		guard = Guard {
			pos = start_pos,
			dir = .North,
		}

		num_walked: int
		for is_inside(&new_level, guard.pos) {
			next_pos := guard.pos + dir_vecs[guard.dir]

			if rune_at(&new_level, next_pos) == '#' {
				guard.dir = turn_right(guard.dir)
				continue
			}

			guard.pos = next_pos
			num_walked += 1
			if num_walked > new_level.width * new_level.height {
				total += 1
				break
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
	testing.expect_value(t, part1(data), 41)
}

@(test)
test_part2 :: proc(t: ^testing.T) {
	data, ok := os.read_entire_file("ex.txt")
	if !ok do return
	defer delete(data)
	testing.expect_value(t, part2(data), 6)
}
