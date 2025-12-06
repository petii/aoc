#day1.py

example= '''L68
			L30
			R48
			L5
			R60
			L55
			L1
			L99
			R14
			L82'''.split('\n')

def rotator(starts, action, onTurnOver):
	direction = action[0]
	amount = int(action[1:])
	newPos = starts
#print(action)
	for i in range(1, amount+1):
		newPos = (newPos + (1 if direction == 'L' else -1)) % 100
		if newPos == 0:
#print('click', i)
			onTurnOver()
#print(starts, ' -> ', newPos)
	return newPos


def counterIncrease():
	global counter
	counter += 1

with open('inputs/day1.txt') as f:
	counter = 0
	position = 50
	for line in example:
		position = rotator(position, line.strip(), counterIncrease)
	print(counter)

	counter = 0
	position = 50
	for line in f.readlines():
		position = rotator(position, line.strip(), counterIncrease)
	print(counter)