
with open("./day1.txt", "r") as file: 
    content = file.read()

content = list(content)

floor = 0
base_pos = 0

for pos, brace in enumerate(content): 
    if brace == '(': 
        floor += 1
    else:
       floor -= 1
    if floor < 0: 
       base_pos = pos + 1  
       break

# print(floor)
print(base_pos)