package main

import (
  "bufio"
  "fmt"
  "os"
  "strings"
  "strconv"
  "sort"
)


func main() {
  file, err := os.Open("./day1.txt")
	if err != nil {
		fmt.Println("Error opening file:", err)
		return
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)

  leftList := make([]int, 0)
  rightList := make([]int, 0)
  
	for scanner.Scan() {
    a := scanner.Text()
    nums := strings.Split(a,"   ") 
    
    left, _ := strconv.Atoi(nums[0])
    right, _ := strconv.Atoi(nums[1])

    leftList = append(leftList, left) 
    rightList = append(rightList, right) 
	}

  sort.Ints(leftList)
  sort.Ints(rightList)
  
  ans := 0

  for i := len(leftList) - 1; i >= 0; i-- {
    diff := leftList[i] - rightList[i]
    if diff < 0 {
      diff = -diff
    }
    ans += diff
  }

  fmt.Printf("%d\n", ans)

  // part 2
  
  countMap := make(map[int]int)

  for _, leftVal := range leftList {
    if _, yes := countMap[leftVal]; yes {
      continue
    }
    for _, rightVal := range rightList {
      if leftVal == rightVal {
        _, yes := countMap[leftVal]
        if yes {
          countMap[leftVal] += 1 
        } else {
          countMap[leftVal] = 1
        }
      }
    }
    _, yes := countMap[leftVal]
    if !yes {
      countMap[leftVal] = 0 
    }
  }
  
  ans2 := 0

  for _, val := range leftList {
    ans2 += val * countMap[val] 
  }

  fmt.Printf("%d", ans2)
 
	if err := scanner.Err(); err != nil {
		fmt.Println("Error reading file:", err)
	}  
}

