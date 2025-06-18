// https://leetcode.com/problems/divide-array-into-arrays-with-max-difference

import (
	"slices"
)

func divideArray(nums []int, k int) [][]int {
	n := len(nums)
	slices.Sort(nums)

	ans := make([][]int, n/3)
	for i := 0; i < n; i += 3 {
		x, y, z := nums[i], nums[i+1], nums[i+2]
		if z-x > k {
			return nil
		}
		ans[i/3] = []int{x, y, z}
	}

	return ans
}