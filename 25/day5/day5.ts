let input = Bun.file("day5.txt")

input = await input.text()
let [fresh_range, items] = input.trim().split("\n\n").map(v => v.split("\n"))

//let ans = 0

//for(let i = 0; i < items.length; i++) {
//    const item = +items[i]
//    for(let range of fresh_range) {
//        const [start, end] = range.split("-")
//        if (item >= +start && item <= +end) {
//            ans += 1
//            break
//        }
//    }
//}
//

fresh_range = fresh_range.map(v => v.split("-").map(e => +e)).sort((a, b) => a[0] - b[0]) 

let [currStart, currEnd] = fresh_range[0]

let ans = 0

for(let i = 1; i < fresh_range.length; i++) {
    const [start, end] = fresh_range[i]
    if (start > currEnd) {
        ans += currEnd - currStart + 1
        currStart = start
        currEnd = end
    } else {
        currEnd = Math.max(currEnd, end)
    }
}

ans += currEnd - currStart + 1

console.log(ans)

