let input = Bun.file("day4.txt")

input = await input.text()
input = input.trim().split("\n").map((row) => row.split(""))

let ans = 0, last = undefined

//for(let i = 0; i < input.length; i++) {
//    const row = input[i]
//    for (let j = 0; j < row.length; j++) {
//        if (input[i][j] == ".") continue
//        const dirs = [[0, 1], [1, 0], [1, 1], [-1, -1], [0, -1], [-1, 0], [-1, 1], [1, -1]]
//        let count = 0
//        for (let [x, y] of dirs) {
//            const dx = i + x, dy = j + y
//            if (((0 <= dx) && (dx < input.length)) && ((0 <= dy) && (dy < row.length))) {
//                if (input[dx][dy] == "@") count += 1
//            }
//        }
//        if (count < 4) ans += 1
//    }
//}

while (last != ans) {
    last = ans
    for(let i = 0; i < input.length; i++) {
        const row = input[i]
        for (let j = 0; j < row.length; j++) {
            if (input[i][j] == ".") continue
            const dirs = [[0, 1], [1, 0], [1, 1], [-1, -1], [0, -1], [-1, 0], [-1, 1], [1, -1]]
            let count = 0
            for (let [x, y] of dirs) {
                const dx = i + x, dy = j + y
                if (((0 <= dx) && (dx < input.length)) && ((0 <= dy) && (dy < row.length))) {
                    if (input[dx][dy] == "@") count += 1
                }
            }
            if (count < 4) {
                ans += 1
                input[i][j] = "."
            }
        }
    }
}

console.log(ans)

