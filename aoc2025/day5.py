#day5.py
from pprint import pprint

example = '''3-5
			 10-14
			 16-20
			 12-18

			 1
			 5
			 8
			 11
			 17
			 32
'''

def clean_input(db_input):
	return list(map(lambda line: line.strip(), db_input.split('\n')))

def parse_range_string(range_string):
	ranges = range_string.split('-')
	return tuple(map(int, ranges))

class CafeteriaInventory:
	def __init__(self, db_input, pretty=False, debug=False):
		self.raw_input = db_input
		db = clean_input(db_input)
		split_idx = db.index('')
		self.ranges = list(map(parse_range_string, db[:split_idx]))
		self.ingredients = list(map(int, filter(lambda l: l != '', db[split_idx+1:])))
		self.pretty = pretty
		self.debug = debug
		if pretty:
			print({"ranges": self.ranges, "ingredients": self.ingredients})

	def list_ingredients(self):
		for i in self.ingredients:
			i_in_r = []
			for r in self.ranges:
				if r[0] <= i <= r[1]:
					i_in_r.append(r)
			yield (i, i_in_r)


	def merge_ranges(self):
		has_overlap = True
		ranges_copy = self.ranges
		while has_overlap:
			has_overlap = False
			new_ranges = set()

			for r in ranges_copy:
				range_start = r[0]
				range_end = r[1]

				merge_candidate_ranges = []
				for r2 in ranges_copy:
					if r2[0] <= range_start <= r2[1] or r2[0] <= range_end <= r2[1]:
						merge_candidate_ranges.append(r2)

				if len(merge_candidate_ranges) > 1:
					has_overlap = True

				starts = map(lambda r: r[0], merge_candidate_ranges)
				ends = map(lambda r: r[1], merge_candidate_ranges)

				new_range = (min(starts), max(ends))
				new_ranges.add(new_range)
				if self.debug:
					print(f'\t[{range_start}-{range_end}]\t{merge_candidate_ranges} -> {new_range}')

			ranges_copy = list(new_ranges)
			if self.debug:
				print(f'\tloop? {has_overlap}, {new_ranges},{ranges_copy}')

		if self.debug:
			print(f'\tmerged=\t{ranges_copy}')
		newCi = CafeteriaInventory(self.raw_input, self.pretty, self.debug)
		newCi.ranges = ranges_copy
		return newCi


def part1(ci):
	return len(list(filter(lambda rs: len(rs[1]) > 0 ,ci.list_ingredients())))

def part2(ci):
	ci = ci.merge_ranges()
	return sum(map(lambda r: r[1] - r[0] + 1, ci.ranges))

ci = CafeteriaInventory(example, pretty=True, debug=True)
print('example')
print('part 1 =', part1(ci))
print('part 2 =', part2(ci))

print('input')
with open('inputs/day5.txt') as f:
	ci = CafeteriaInventory(f.read())
	print('part 1 =', part1(ci))
	print('part 2 =', part2(ci))