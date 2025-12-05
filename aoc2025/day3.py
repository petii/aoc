# day3.py

from pprint import pprint
from itertools import combinations

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
		def get_candidate(bank, top_x = None):
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

			start_pos = 0
			merged_mask = [-1] * len(bank_jolts)
			for d in digits_desc[:top_x] if top_x is not None else digits_desc:
				mask = digit_masks[d]

				result_lenght = sum(1 if v >= 0 else 0 for v in merged_mask)
				if result_lenght >= batteries:
					break;

				for i,v in enumerate(reversed(mask)):
					rev_i = len(mask) - i - 1
					result_lenght = sum(1 if v >= 0 else 0 for v in merged_mask)
					if i >= start_pos and merged_mask[rev_i] < v and result_lenght < batteries:
						merged_mask[rev_i] = v

				for i, v in enumerate(mask):
					if v < 0:
						continue

					leftover_candidates = sum(1 if j < d else 0 for j in bank_jolts[i:])
					# print(d, mask, leftover_candidates, result_lenght)
					if leftover_candidates > batteries - result_lenght:
						start_pos = i
			
			res =  ''.join([str(v) if v >= 0 else '' for v in merged_mask])
			if not quiet:
				print(f'\t[{bank}]',
					digits_desc[:top_x] if top_x is not None else digits_desc,
					len(res), res)
			# pprint(digit_masks)

			if len(res) < batteries:
				raise Exception('whoops, not enough digits')
			return res

		mask_candidate = get_candidate(bank)

		# if len(mask_candidate) < batteries * 2:
			# 2x C x does not scale amazingly, but should be fast enough for now
		candidates = map(lambda c: int(''.join(list(c))), combinations(mask_candidate, batteries))
		# else:
		# 	# too many repeating numbers - filter it down
		# 	top = 1
		# 	jolts = list(map(int, list(bank)))
		# 	max_jolt = max(jolts)
		# 	max_idx = jolts.index(max_jolt)
		# 	new_bank = bank[max_idx:]
		# 	# if len(new_bank) <
		# 	new_candidate = []
		# 	if not quiet:
		# 		print(f'\t[{bank}] top={top} {max_jolt} [{new_bank}]') 
		# 	try:
		# 		new_candidate = get_candidate(new_bank,top)
		# 	except:
		# 		pass
		
		# 	while len(new_candidate if new_candidate is not None else []) < batteries:
		# 		top += 1
		# 		if not quiet:
		# 			print(f'loop\t[{bank}] top={top} {max_jolt} [{new_bank}]') 
		# 		try:
		# 			new_candidate = get_candidate(new_bank,top)
		# 		except:
		# 			print('threw for top =', top)
		# 			try:
		# 				new_candidate = get_candidate(bank,top)
		# 			except:
		# 				pass
		# 				new_candidate = None
		# 			# we're increasing the numbers anyway
		# 		finally:
		# 			if not quiet:
		# 				print(f'loop\t[{bank}]', len(new_candidate if new_candidate is not None else []), new_candidate)
						
		# 		if top > 9:
		# 			break

		# 	if len(new_candidate) >= batteries * 2:
		# 		# still too much - remove the smallest numbers on the biggest decimal place until good
		# 		candidate_jolts = list(map(int, list(new_candidate)))
		# 		min_jolt = min(candidate_jolts)
		# 		newest_candidate_list = list(new_candidate)
		# 		while len(newest_candidate_list) > batteries:
		# 			newest_candidate_list.pop(newest_candidate_list.index(str(min_jolt)))
		# 		new_candidate = ''.join(newest_candidate_list)

		# 		if not quiet:
		# 			print(f'fin = [{bank}]', new_candidate)


		# 	candidates = map(lambda c: int(''.join(list(c))), combinations(new_candidate, batteries))
		# 	mask_candidate = new_candidate

		if not quiet:
			print(f'res=\t[{bank}]', len(mask_candidate), mask_candidate)
		# candidates.append(int(''.join([str(v) if v >= 0 else '' for v in merged_mask])))

	
	if not quiet:
		print(f'\t[{bank}] {candidates}')
	return max(candidates)


for bank in batt_banks:
	print(bank, '->', max_joltage(bank, 12, quiet=False))

print('example part1',sum(map(max_joltage, batt_banks)))
print('example part2',sum(map(lambda b: max_joltage(b,12), batt_banks)))

with open('inputs/day3.txt') as f:
	input_banks = list(filter(
		lambda l: l != '',
		map(lambda l: l.strip(), f.readlines())))
	print('part1',
		sum(
			map(max_joltage, 
				input_banks)))
	print('part2',
		sum(
			map(lambda b: max_joltage(b,12, quiet=False), 
				input_banks)))