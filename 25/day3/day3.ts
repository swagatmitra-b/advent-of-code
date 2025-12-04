let input = Bun.file("day3.txt")

input = await input.text()

input = input.trim().split("\n")

let ans = 0

//for(let i = 0; i < input.length; i++) {
//    const bank = input[i]
//    let a = 0, b = 0
//    for(let j = 0; j < bank.length; j++) {
//        const num = +bank[j]
//        if (num > a && j < bank.length - 1) { 
//            a = num 
//            b = +bank[j + 1]
//        } else if ((a >= num && num > b) || (num > a && j == bank.length - 1)) b = num 
//    }
//    ans += (10*a + b)
//}
//console.log(ans)

for (let bank of input) {
    let num = ""
    let start = 0
    for (let i = 12; i > 0; i--) {
        const end = bank.length - i
        let max = 0
        for(let j = start; j <= end; j++) {
            if (+bank[j] > max) {
                max = +bank[j]
                start = j + 1
            }
        }
        num += max
    }
    ans += +num
}

console.log(ans)
