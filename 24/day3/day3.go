package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
)

func main() {
	file, err := os.Open("./day3.txt")
	if err != nil {
		fmt.Println("Error opening file:", err)
		return
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)
  
	input := ""
	isMul := ""
  
  isDont := false
  
	ans := 0

	for scanner.Scan() {
		input += scanner.Text()
	}

	for i, char := range input {
    
    if i+6 < len(input) && string(input[i:i+7]) == "don't()" {
			isDont = true
			i += 6 
			continue
		}

		if i+3 < len(input) && string(input[i:i+4]) == "do()" {
			isDont = false
			i += 3 
			continue
		}
    
    if isDont {
      continue
    }
    
		if (char == 'm' && len(isMul) == 0) ||
			(char == 'u' && len(isMul) == 1 && isMul[0] == 'm') ||
			(char == 'l' && len(isMul) == 2 && isMul[1] == 'u') {
			isMul += string(char)
		}

		if isMul == "mul" && i+1 < len(input) && input[i+1] == '(' {
			check(i+2, input, &ans)
		}

		if !(char == 'm' || char == 'u' || char == 'l') {
			isMul = ""
		}
	}

	if err := scanner.Err(); err != nil {
		fmt.Println("Error reading file:", err)
	}

	fmt.Println(ans)
}

func check(idx int, input string, ans *int) {
	first, second := "", ""

	for idx < len(input) && input[idx] != ',' {
		if !isNum(input[idx]) {
			return
		}
		first += string(input[idx])
		idx++
	}
  
	idx++
  
	for idx < len(input) && input[idx] != ')' {
		if !isNum(input[idx]) {
			return
		}
		second += string(input[idx])
		idx++
	}
  
	num1, err1 := strconv.Atoi(first)
	num2, err2 := strconv.Atoi(second)
	if err1 != nil || err2 != nil {
		fmt.Println("Error in converting first or second number")
		return
	}

	*ans += num1 * num2
}

func isNum(char byte) bool {
	_, err := strconv.Atoi(string(char))
	return err == nil
}

