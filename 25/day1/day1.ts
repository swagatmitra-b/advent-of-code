let input = Bun.file("day1.txt")

input = await input.text()

input = input.split("\n")

let dial = 50
let ans = 0
let MOD = 100

// pt: 1

//for(let i = 0; i < input.length - 1; i++) {
//    const [direction, steps] = input[i].split(/(\d+)/, 2)
//    if (direction == "R") dial = (dial + Number(steps)) % MOD
//    else {
//        const offset = (dial - Number(steps)) % MOD
//        if (offset >= 0) dial = offset 
//        else dial = (100 + offset) 
//    }
//
//    if (dial == 0) ans += 1
//}

for(let i = 0; i < input.length - 1; i++) {
    const [direction, stepsStr] = input[i].split(/(\d+)/, 2)
    const steps = Number(stepsStr)

    if (direction === "R") {
        const afterTurn = dial + steps
        const firstZero = ((MOD - dial) % MOD) || MOD
        if (steps >= firstZero) {
            ans += Math.floor((steps - firstZero) / MOD) + 1
        }
        dial = afterTurn % MOD
    }
    else { 
        const afterTurn = dial - steps
        const firstZero = dial === 0 ? MOD : dial
        if (steps >= firstZero) {
            ans += Math.floor((steps - firstZero) / MOD) + 1
        }
        dial = ((afterTurn % MOD) + MOD) % MOD
    }
}

console.log(ans)
