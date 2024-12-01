
with open("./day2.txt", "r") as file: 
    content = file.read()

content = [x.split("x") for x in content.split("\n")]

content = [list(map(int, exp)) for exp in content]

total = 0

for idx, area in enumerate(content): 
    # surface = 2 * area[0] * area[1] + 2 * area[1] * area[2] + 2 * area[2] * area[0]
    surface = area[0] * area[1] * area[2]

    min_area = min(area)
    area.remove(min_area)
    sec_min_area = min(area)

    # extra = min_area * sec_min_area
    extra = min_area * 2 + sec_min_area * 2

    total += extra + surface

print(total)