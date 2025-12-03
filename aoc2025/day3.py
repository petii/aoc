# day3.py

from pprint import pprint

example = '''
987654321111111
811111111111119
234234234234278
818181911112111
'''

batt_banks = list(
	map(lambda l: l.strip(),
		filter(lambda l: l != '',example.split('\n'))))

def max_joltage(bank, batteries=2, quiet = True):
	candidates = []
	if batteries == 2:
		first_digit = None
		second_digit = None
		for b in bank:
			j = int(b)
			# print(f'\t\t[{j}] {first_digit}{second_digit}',end=' ')
			if first_digit == None or j > first_digit:
				# print('branch 1')
				if first_digit != None:
					candidates.append(int(f'{first_digit}{j}'))
				first_digit = j
				second_digit = None
				continue

			if first_digit != None and (second_digit == None or j > second_digit):
				# print('branch 2',end=' ')
				second_digit = j

			if first_digit != None and second_digit != None:
				# print('branch 3',end=' ')
				candidates.append(int(f'{first_digit}{second_digit}'))
			# print('')
	else:
		bank_jolts = list(map(int, list(bank)))
		digit_masks = {}
		for j in set(bank_jolts):
			mask = [-1] * len(bank_jolts)
			for i, v in enumerate(bank_jolts):
				if v == j:
					mask[i] = v
			digit_masks[j] = mask
			# print(j,'->',mask)

		digits_in_dict = digit_masks.keys()
		digits_desc = sorted(digits_in_dict, reverse=True)
		list(
			range(max(digit_masks.keys()), 
				  min(digit_masks.keys())-1,
				  -1))
		if not quiet:
			print(f'\t[{bank}]', digits_desc)
		# pprint(digit_masks)

		merged_mask = [-1] * len(bank_jolts)
		for d in digits_desc:
			mask = digit_masks[d]
			for i,v in enumerate(reversed(mask)):
				rev_i = len(mask) - i - 1
				result_lenght = sum(1 if v >= 0 else 0 for v in merged_mask)
				if merged_mask[rev_i] < v and result_lenght < batteries:
					merged_mask[rev_i] = v

		if not quiet:
			print(f'\t[{bank}]', merged_mask)

		candidates.append(int(''.join([str(v) if v >= 0 else '' for v in merged_mask])))

	if not quiet:
		print(f'\t[{bank}] {set(candidates)}')
	return max(candidates)


for bank in batt_banks:
	print(bank, '->', max_joltage(bank, 12, quiet=False))

print('part1',sum(map(max_joltage, batt_banks)))
print('part2',sum(map(lambda b: max_joltage(b,12), batt_banks)))

with open('inputs/day3.txt') as f:
	print(
		sum(
			map(max_joltage, 
				filter(
					lambda l: l != '',
					map(lambda l: l.strip(), f.readlines())))))