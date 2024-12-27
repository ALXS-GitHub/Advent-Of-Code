# Day 4

# -- Part 1 --

lines = []

with open('day04/input.txt') as f:
    lines = f.read().split('\n')
    
lines = [item.split(',') for item in lines]

def fully_contains(a1:str,a2:str)->bool:
    """ indicates if one of the two assignment is fully contained in the other one

    Args:
        a1 (str): assignment 1 'x1-y1'
        a2 (str): assignment 2 'x2-y2'

    Returns:
        bool: 
    """
    
    x1,y1 = a1.split('-')
    x2,y2 = a2.split('-')
    x1,y1,x2,y2 = int(x1),int(y1),int(x2),int(y2)

    
    if (x1 <= x2) and (y1 >= y2):
        return True
    elif (x1 >= x2) and (y1 <= y2):
        return True
    
    else: 
        return False
    
    # return (x1 <= x2 and y1 >= y2) or (x1 >= x2 and y1 <= y2)

print(fully_contains('1-9','2-10'))

# @ calcul du nombre de paires dans le cas donné

number = 0

for x in lines:
    if fully_contains(x[1],x[0]):
        number += 1

print(number)


# -- Part 2 --

def overlapping(a1:str,a2:str)->bool:
    """ indicates if the two assignments are overlapping

    Args:
        a1 (str): assignment 1
        a2 (str): assignment 2

    Returns:
        bool: 
    """
    
    x1,y1 = a1.split('-')
    x2,y2 = a2.split('-')
    
    x1,y1,x2,y2 = int(x1),int(y1),int(x2),int(y2)
    
    if x1 <= x2 <= y1 or x1 <= y2 <= y1:
        return True
    
    if x2 <= x1 <= y2 or x2 <= y1 <= y2:
        return True
    
    return False

print(overlapping('9-9','3-10'))

# @ calcul du nombre de paires dans le cas donné

number = 0

for x in lines:
    if overlapping(x[0],x[1]):
        number += 1
        
print(number)