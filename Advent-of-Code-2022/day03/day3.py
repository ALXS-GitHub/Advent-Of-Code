# Day 3

# -- Part 1 --

def priority(c:str)->int:
    code = ord(c)

    code = code % 65 + 1
    
    if code <= 26:
        code += 26
        
    elif code >= 33:
        code -= 32
        
    return code

print(priority('Z'))


with open('day03/input.txt') as f:
    lines = f.read().split('\n')
    

def find_same_letter(s:str)->str:
    """ find the letter that appears in both sides of the string

    Args:
        s (str): on rucksack

    Returns:
        str: the letter found
    """
    
    n = len(s)
    p1,p2 = s[:n//2], s[n//2:]
    letter = ''

    for c in p1:
        if c in p2:
            letter = c
            break

    return letter

# @ on cherche la letter dans tous les sac et on ajoute la priorité à la somme

somme = 0

for l in lines:
    c = find_same_letter(l)
    somme += priority(c)
    
print(somme)

# -- Part 2 --

def find_badge(e1:str,e2:str,e3:str)->str:
    """ trouve le badge d'un groupe d'elfes

    Args:
        e1 (str): elfe 1
        e2 (str): elfe 2
        e3 (str): elfe 3

    Returns:
        str: badge
    """
    
    for a in e1:
        
        if a in e2 and a in e3:
            letter = a 
            break
    
    return letter

# @ on cherche le badge de tous les groupes:

somme = 0

for i in range(0,len(lines),3):
    c = find_badge(lines[i],lines[i+1],lines[i+2])
    somme += priority(c)
    
print(somme)