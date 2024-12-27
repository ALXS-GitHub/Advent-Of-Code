# Day 1

with open("day01/input.txt") as f:
    lignes = f.read().split('\n')
    
lignes = [int(x) if not x == '' else '' for x in lignes]
# print(lignes)

# @ somme des calories de chaque elfe

elfes = []
prev_i = 0

for i in range(len(lignes)):
    if lignes[i] == '':
        
        s = lignes[prev_i:i]

        elfes.append(sum(s))
        prev_i = i + 1
        
# print(elfes)

x = max(elfes)

print(x)

# -- Part 2 --

i1 = elfes.index(x)
print(i1)

elfes.pop(i1)

x2 = max(elfes)
i2 = elfes.index(x2)
elfes.pop(i2)

x3 = max (elfes)

print(x + x2 + x3)