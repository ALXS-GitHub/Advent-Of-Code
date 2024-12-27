# Day 6

# -- Part 1 --

with open('day06/input.txt') as f:
    line = f.read()

# print(line)



def find_marker_id(line:str)->int:
    """ find the numbers of characters before the marker

    Args:
        line (str): datastream buffer

    Returns:
        int: index
    """
    max_id = len(line)
    
    if max_id < 3: # si la taille de la str est inférieure à 4
        return -1

    for i in range(3,max_id):
        
        last_four = sorted(line[i-3:i+1]) # les quatres derniers caractères
        
        last_four_unique = sorted(set(last_four))
        
        if last_four == last_four_unique:
            return i + 1
        
    return -1 # cas où on ne trouve pas de marker

assert find_marker_id('bvwbjplbgvbhsrlpgdmjqwftvncz') == 5
assert find_marker_id('nppdvjthqldpwncqszvftbrmjlhg') == 6
assert find_marker_id('nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg') == 10
assert find_marker_id('zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw') == 11

print(find_marker_id(line)) # 1640

# -- Part 2 --

def find_message_marker_id(line:str)->int:
    """ trouve le nombre de caractères avant le message marker

    Args:
        line (str): signal

    Returns:
        int: index
    """
    
    max_id = len(line)
    
    if max_id < 13: # si le signal est de longueur inférieure à 14
        return -1
    
    for i in range(13,max_id):
        
        last_14 = sorted(line[i-13:i+1])
        
        last_14_unique = sorted(set(last_14))
        
        if last_14 == last_14_unique:
            return i + 1
        
    return -1

assert find_message_marker_id('mjqjpqmgbljsphdztnvjfqwrcgsmlb') == 19
assert find_message_marker_id('bvwbjplbgvbhsrlpgdmjqwftvncz') == 23
assert find_message_marker_id('nppdvjthqldpwncqszvftbrmjlhg') == 23
assert find_message_marker_id('nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg') == 29
assert find_message_marker_id('zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw') == 26

print(find_message_marker_id(line)) # 3613