#day2.py

example = '''11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124'''

def check_patterns(id_part, pattern):
#print('\t', id_part, pattern)
	if id_part == pattern:
		return True
	if len(id_part) == 0:
		return False
	return id_part[0:len(pattern)] == pattern and check_patterns(id_part[len(pattern):], pattern)

def is_invalid_id(id):
	halfpoint = len(id)//2

	result = False
	for pattern_length in range(1,halfpoint+1):
		pattern = id[0:pattern_length]
		result = check_patterns(id[pattern_length:], pattern)
#print(f '[{id}]', pattern_length, pattern, result)
		if result:
			break

#result = id[0 : halfpoint] == id[halfpoint : ]
	if result:
		print('\t',id, halfpoint, pattern)
	return result

def invalid_ids_in_range(start, end):
	print('range',start, '-',end)
	startNum = int(start)
	endNum = int(end)
	ids = []
	for id in range(startNum, endNum+1):
		if is_invalid_id(str(id)):
			ids.append(id)
	return ids

def to_range(range_str):
	return range_str.split('-')

id_sum = 0
for range_str in example.split(','):
	id_sum += sum(invalid_ids_in_range(*to_range(range_str)))
print(id_sum)

with open('inputs/day2.txt') as f:
	id_sum = 0
	line = f.readlines()[0]
	for range_str in line.split(','):
		id_sum += sum(invalid_ids_in_range(*to_range(range_str)))
	print(id_sum)
