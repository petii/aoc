# day4.py
from pprint import pprint

example = '''
..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.
'''


class PrintDepartment:
	def __init__(self, floor, pretty=False, debug=False):
		# cleaned_floor = filter(lambda l: l.strip() != '', floor.split('\n'))
		cleaned_floor = floor
		self.map = [list(row) for row in cleaned_floor]
		self.width = len(self.map[0])
		self.heigth = len(self.map)
		self.debug = debug
		self.pretty = pretty
		if pretty:
			pprint(self.map)

	def neighbours(self, row, col):
		col_from, col_to = max(0, col - 1), min(self.width - 1, col + 1)
		row_from, row_to = max(0, row - 1), min(self.heigth - 1, row + 1)

		neighbouring_cells = []

		for i in range(row_from, row_to+1):
			for j in range(col_from,col_to+1):
				if not (i == row and j == col) :
					neighbouring_cells.append(self.map[i][j])

		if self.debug:
			pprint({"args": (row, col),
				"col_range":(col_from, col_to), 
				"row_range":(row_from, row_to), 
				"neighbours": neighbouring_cells})

		for c in neighbouring_cells:
			yield c

	def cells(self):
		for i, row in enumerate(self.map):
			for j, col in enumerate(row):
				yield ((i,j), col)

	def remove(self, *args):
		newFloor = self.map
		for row, col in args:
			newFloor[row][col] = '.'
		return PrintDepartment(newFloor, debug=self.debug, pretty=self.pretty)


def print_department(floor, pretty=False, debug=False):
	cleaned_floor = filter(lambda l: l.strip() != '', floor.split('\n'))
	return PrintDepartment(cleaned_floor, pretty, debug)


def part1(pd):
	rolls_of_paper = filter(lambda r: r[1] == '@', pd.cells())
	paper_neighbour_count = map(
		lambda cell: sum(
			1 if neighbour == '@' else 0 
			for neighbour in pd.neighbours(*cell[0])), 
		rolls_of_paper)
	less_than_four_neighbours = filter(lambda count: count < 4, paper_neighbour_count)
	return len(list(less_than_four_neighbours))

def part2(pd):
	removed_count = 0
	remove_candidates = ['init']
	while len(remove_candidates) > 0:
		rolls_of_paper = filter(lambda r: r[1] == '@', pd.cells())
		with_paper_neighbour_count = map(
			lambda cell: (cell[0],
				sum(
					1 if neighbour == '@' else 0 
					for neighbour in pd.neighbours(*cell[0])),
				),
			rolls_of_paper)
		remove_candidates = list(map(
			lambda cell: cell[0],
			filter(lambda cell: cell[1] < 4, with_paper_neighbour_count)))

		pd = pd.remove(*remove_candidates)
		removed_count += len(remove_candidates)

	return removed_count


print('example')
pd = print_department(example, pretty=True)
print(pd, pd.width, pd.heigth)
print('part 1 = ', part1(pd))
print('part 2 = ', part2(pd))

print('input')
with open('inputs/day4.txt') as f:
	pd = print_department(f.read())
	print(pd, pd.width, pd.heigth)
	print('part 1 = ', part1(pd))
	print('part 2 = ', part2(pd))

