

with open('./day6.txt', 'r') as file: 
    content = file.read()

time, distance = [list(filter(lambda x: x != '', [x for x in i.split(" ")])) for i in content.split('\n')]

time.pop(0)
distance.pop(0)

time = list(map(int, time))
distance = list(map(int, distance))

ways = 1

for (mil, dist) in zip(time, distance): 
    possibles = []
    for i in range(1, mil): 
        speed = i
        travelled = (mil - i) * speed
        if travelled > dist: 
            possibles.append(i)
    # print(possibles)
    ways *= len(possibles)
    print(ways)

def part2(a, b): 
    num1 = int("".join(list(map(str, a))))
    num2 = int("".join(list(map(str, b))))
    possibles2 = []
    for i in range(1, num1): 
        speed = i
        travelled = (num1 - i) * speed
        if travelled > num2: 
            possibles2.append(i)
    print(len(possibles2))

part2(time, distance)

# print(time, distance)