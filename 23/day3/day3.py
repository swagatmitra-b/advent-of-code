

with open('./day3.txt', 'r') as file:
    content = file.read()

def convert_to_int(string: str):
    if string.isdigit():
        return int(string)
    return None

content = [list(x) for x in content.split('\n')]

symbols = ["*", "/", "-", "&", "@", "#", "$", "=", "%", "+"]
neighbors = [
    [1, 1], [1, -1], [-1, 1], [-1, -1],
    [0, 1], [1, 0], [0, -1], [-1, 0]
]

total_sum = 0


sym = False
num_set = []

print(len(content), len(content[0]))

for r_idx, row in enumerate(content):
    first = False
    for c_idx, col in enumerate(row):
        a = convert_to_int(col)
        if a or a == 0:
            num_set.append(a)
            if not content[r_idx][c_idx - 1] or not convert_to_int(content[r_idx][c_idx - 1]):
                first = True
            for x, y in neighbors:
                newX, newY = x + r_idx, y + c_idx
                if 0 <= newY < len(row) and 0 <= newX < len(content):
                    if content[newX][newY] in symbols:
                        sym = True
                    # if content[newX][newY] in symbols or first:
                    #     print(num_set, content[newX][newY], first, sym, a)
            if c_idx + 1 >= len(row) and sym:
                res_str = ''.join(map(str, num_set))
                res_num = int(res_str)
                # print(res_num, num_set)
                with open('./output.txt', 'a') as output:
                   output.write(f"{res_str}\n")
                total_sum += res_num
                num_set.clear()
                sym = False
        else:
            # print(col, sym, num_set)
            if sym and len(num_set):
                res_str = ''.join(map(str, num_set))
                res_num = int(res_str)
                # print(res_num, num_set)
                with open('./output.txt', 'a') as output:
                    output.write(f"{res_str}\n")
                total_sum += res_num
            num_set.clear()
            sym = False

print("Total Sum:", total_sum)
