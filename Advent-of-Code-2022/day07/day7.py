# Day 7

import re
import sys

sys.setrecursionlimit(10000)

# -- Part 1 --

with open('day07/input.txt') as f:
    lines = f.read().split('\n')
    
    
known_directory_bis = {}
NUMBER_OF_LINES = len(lines)

def size_directory(d:str,min_idx,lines)->int:
    """ taille d'un dossier
    Args:
        d (str): dossier

    Returns:
        int: taille du dossier 
    """
    
    new_command = False;
    size = 0
    
    if not f'$ cd {d}' in lines:
        print(d)
        print(lines[0])
        print(min_idx)
        print(known_directory_bis)
        print('no such directory')
        return size
    
    idx_dir = lines.index(f'$ cd {d}',min_idx) # indice de la ligne de la commande cd 
    
    
    
    
    i = idx_dir
    
    if not lines[i+1] == '$ ls':
        print('no ls')
        return
    
    i += 2
    
    if lines[i][0] == '$': # premier caractère de la ligne $ pour détecter une nouvelle commande
        new_command = True
        return size
    
    while not new_command and i < NUMBER_OF_LINES:
        
        print(i,min_idx)
        
        if i+1 < NUMBER_OF_LINES:
            if lines[i+1][0] == '$': # premier caractère de la ligne suivante $ pour détecter une nouvelle commande
                new_command = True
            
        if re.search('^dir',lines[i]):
            
            new_dir = re.findall('\S+$',lines[i])[0]
            if '/' in list(new_dir):
                print('wtf??')
            
            dir_size = size_directory(new_dir,i,lines)
            
            
            
            size += dir_size
            
        elif re.search('^\d+',lines[i]):
            file_size = int(re.findall('^\d+',lines[i])[0])
            
            size += file_size
            
        i+=1
            
    # on ajoute la valeur dans un dictionnaire
    known_directory_bis[idx_dir] = size
    
    # ! on ajoute un caractère pour ne plus mesurer la taille de ce dossier
    lines[idx_dir] = '# ' + lines[idx_dir]
    
    return size
        
print(size_directory('/',0,lines))        

"""print(lines[16])

known_directory = {}

for i in range(len(lines)):
    if re.search('^\$ cd',lines[i]) and lines[i][-1] != '.':
        d = re.findall('\S*$',lines[i])[0]
        known_directory[i] = size_directory(d,i,lines)
        
delete_size = 0
for key in known_directory:
    if known_directory[key] <= 100000:
        delete_size += known_directory[key]
  
# print(known_directory)      
# print(delete_size)

# -- Part 2 --

L,N = [], []
for key in known_directory:
    L.append(known_directory[key])
    N.append(known_directory_bis[key])
    
print(sorted(L))
print(sorted(N))

disk_used = 0
for key in known_directory:
    disk_used += known_directory[key]
    
    if key not in known_directory_bis:
        known_directory[key] = known_directory_bis[key]
    
print(disk_used,size_directory('/',0,lines))



space_needed = size_directory('/',0,lines) - 40000000

print(space_needed)

potential_folders = []


for key in known_directory_bis:
    if known_directory_bis[key] >= space_needed:
        potential_folders.append(known_directory_bis[key])
        
print(min(potential_folders))
print(sorted(potential_folders))"""