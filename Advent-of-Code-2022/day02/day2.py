# Day 2

with open('day02/input.txt') as f:
    lignes = f.read().split('\n')

# print(lignes)


strategy = [item.split(' ') for item in lignes]

# print(strategy)

def jeu(j1:str,j2:str)->str:
    """ jeu pierre feuille ciseau

    Args:
        j1 (str): joueur 1
        j2 (str): joueur 2

    Returns:
        str: L D W (lose/draw/win)
    """
    
    if (j1,j2) in (('A','X'),('B','Y'),('C','Z')):
        return 'D'
    
    elif (j1,j2) in (('A','Y'),('B','Z'),('C','X')):
        return 'W'
    
    else:
        return 'L'
    
def points(j1:str,j2:str)->int:
    """nombre de points d'une manche

    Args:
        j1 (str): joueur 1
        j2 (str): joueur 2

    Returns:
        int: nombre de points de la manche
    """
    
    p = 0
    
    if jeu(j1,j2) == 'W':
        p += 6
    elif jeu(j1,j2) == 'D':
        p += 3
        
    if j2 == 'X':
        p += 1
    elif j2 == 'Y':
        p += 2
    elif j2 == 'Z':
        p += 3
        
    return p
      
      
somme = 0
  
for i in range(len(strategy)):
    somme += points(strategy[i][0],strategy[i][1])
    
print(somme)    

# -- Part 2 --

def objet(x:str)->int:
    """_summary_

    Args:
        x (str): _description_

    Returns:
        int: _description_
    """
    if x == 'A':
        return 1
    elif x == 'B':
        return 2
    else:
        return 3

def points2(j1,s)->int:
    """_summary_

    Args:
        j1 (_type_): _description_
        s (_type_): stratégie à adopter
    """
    p = 0
    
    if s == 'Y':
        p += 3 + objet(j1)
        
    if s == 'Z':
        pts = (objet(j1)) % 3 + 1 
        p += 6 + pts

    if s == 'X':
        pts = (objet(j1) - 2) % 3 + 1
        p += pts
        
    return p

print(points2('C','Z'))

somme = 0
  
for i in range(len(strategy)):
    somme += points2(strategy[i][0],strategy[i][1])
    
print(somme)  