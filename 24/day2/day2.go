package main

import (
  "bufio"
  "fmt"
  "os"
  "strings"
  "strconv"
)

func main() {
  file, err := os.Open("./day2.txt")
	if err != nil {
		fmt.Println("Error opening file:", err)
		return
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)
  
  reports := make([][]int, 0)

	for scanner.Scan() {
    lines := strings.Split(scanner.Text(), " ")
    report := make([]int, 0)
    for _, num := range lines {
       nInt, _ := strconv.Atoi(num)
       report = append(report, nInt)
    }
    reports = append(reports, report)
	}

  safe_reports := 0
  modified_reports := 0

  for _, report := range reports {
    if isSafe(report) {
      safe_reports += 1
    } 
  }

  fmt.Printf("%d \n", safe_reports)

  // part day2

  for _, report := range reports {
    if modifySafe(report) {
      modified_reports += 1
    }  
  }
  
  fmt.Printf("%d \n", modified_reports)

	if err := scanner.Err(); err != nil {
		fmt.Println("Error reading file:", err)
	}  
}

func absInt(num int) int {
  if num < 0 {
    return -num
  }
  return num
}

func isSafe(report []int) bool {
    status := 2 
    for i := 0; i < len(report) - 1; i++ {
      diff := report[i+1] - report[i]
      if diff == 0 || absInt(diff) > 3 {
        return false
      }
      
      if status == 2 {
        if report[i] < report[i + 1] {
          status = 0
        } else if report[i] > report[i + 1] {
          status = 1
        } else {
          return false
        } 
      } else {
        if (report[i] < report[i + 1] && status == 1) || 
        (report[i] > report[i + 1] && status == 0) {
          return false
        } 
      } 
    }
    return true
}

func modifySafe(report []int) bool {
    for i := 0; i < len(report); i++ {
        new_report := append([]int{}, report[:i]...) 
        new_report = append(new_report, report[i+1:]...) 
        if isSafe(new_report) {
          return true
        }
    }
    return false
}
