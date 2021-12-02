package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"

	//"strconv"
)


func check(e error) {
	if e != nil {
		panic(e)
	}
}

// pretty sure this is only accidentally doing what I want
func rotate_left(slice []int) []int {
	r := len(slice) - 1
	slice = append(slice[1:], slice[:r-1]...)
	return slice
}

func compare(prev []int, cur []int) int {
	var prev_sum = 0
	var cur_sum = 0

	for _, e := range prev {
		prev_sum += e
	}

	for _,e := range cur {
		cur_sum += e
	}

	if prev_sum < cur_sum {
		return 1
	} else {
		return 0
	}

}

func main() {
	f, err := os.Open("day1-1/input.txt")
	check(err)
	defer f.Close()

	scanner := bufio.NewScanner(f)
	scanner.Split(bufio.ScanLines)

	var cnt = 0
	prev := []int {0, 0, 0}
	cur := []int {0, 0, 0}
	var s = ""
	var i = 0
	var newPrev = rotate_left(prev)
	fmt.Println(prev)
	fmt.Println(newPrev)

	for scanner.Scan() {
		if i <= 2 {
			num, err := strconv.Atoi(scanner.Text())
			check(err)
			prev[i] = num
			cur = prev
		} else {
			cur = rotate_left(prev)
			num, err := strconv.Atoi(scanner.Text())
			check(err)
			cur[2] = num

			cnt += compare(prev, cur)
			s = fmt.Sprintf("prev: %v cur: %v", prev, cur)
			fmt.Println(s)
			prev = cur
		}
		i++
	}

	fmt.Println(cnt)
	
}
