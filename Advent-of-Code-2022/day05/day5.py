# Day 5

# $ on import les regex
import re

# -- Part 1 --

with open('day05/input.txt') as f:
    lines = f.read().split('\n')

# % on cherche l'index à partir duquel on passe au instruction 
idx= lines.index('')

crates = lines[:idx-1]
instructions = lines[idx+1:]

cratesbis = []
indices = [1,5,9,13,17,21,25,29,33]

for i in range(9):
    temp = []
    for j in range(len(crates)):
        
        x = crates[j][indices[i]]
        
        if x != ' ':
        
            temp.append(x)
            
            
    temp.reverse()
    cratesbis.append(temp)
    
# print(cratesbis)

crates = [[a for a in b] for b in cratesbis]



# % fonction qui execute les instructions

def move(n:int,a:int,b:int)->None:
    """ effectue le move

    Args:
        n (int): nombre de caisses à déplacer
        a (int): position initiale
        b (int): position finale
    """
    
    n,a,b = int(n),int(a),int(b)
    
    if n <= 1:
        
        top = crates[a].pop()
        crates[b].append(top)
        
    else:
        
        top = crates[a].pop()
        crates[b].append(top)
        move(n-1,a,b)


# @ read instructions :

moves = []

for x in instructions:
    
    steps = re.findall("\d+",x)
    
    moves.append(steps)

# print(moves)

# @ on effectue les instructions:

for x in moves:
    
    move(x[0], int(x[1]) - 1, int(x[2]) - 1)
    
# print(crates)

# @ affichage du message

def message():
    """affiche le message
    """
    c = crates[:]
    s = ''
    
    
    for x in c:
        e = x.pop()
        s += e
    
    return s

print(message())

# -- Part 2 --



crates = [[a for a in b] for b in cratesbis]

def move2(n:int,a:int,b:int)-> None:
    """ fonction move CrateMover 9001

    Args:
        n (int): nombre de caisses à déplacer
        a (int): position initiale
        b (int): position finale
    """
    
    taille_a = len(crates[a])
    
    bottom,top = crates[a][:taille_a - n], crates[a][taille_a - n:]
    
    crates[a] = bottom
    crates[b] = crates[b] + top


# @ on effectue les instructions:

for x in moves:
    
    move2(int(x[0]), int(x[1]) - 1, int(x[2]) - 1)
    
print(crates)
print(message())