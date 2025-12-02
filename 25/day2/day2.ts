let input = Bun.file("day2.txt")

input = await input.text()

input = input.split(",")

let ans = 0

//for (let i = 0; i < input.length; i++) {
//    const [start, end] =  input[i].split("-")
//    for (let j = +start; j <= +end; j++) {
//         let strID = j.toString()
//         if (strID.length % 2 != 0) continue
//         if (strID.slice(0, strID.length/2) == strID.slice(strID.length/2, strID.length)) {
//            ans += j
//        }
//    }
//}

//console.log(ans)
const s = new Set()

for (let i = 0; i < input.length; i++) {
    const [start, end] =  input[i].split("-")
    for (let j = +start; j <= +end; j++) {
        let strID = j.toString()
        for (let k = 1; k < strID.length; k++) {
            if (strID.length % k == 0) {
                const target = strID.slice(0, k).repeat(strID.length / k)
                if ((target == strID) && !s.has(target)) {
                    ans += +strID
                    s.add(target)
                }
            }
        }
    }
}
console.log(ans)
