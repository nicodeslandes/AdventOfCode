from typing import List


def apply_mask(state, addr, mask: str, value: int):
    v = 0
    #curr = 1

    for i in range(len(mask)):
        ch = mask[-i-1]
        if ch == '1':
            v += 1 << i
        elif ch == 'X':
            v += (value & 1) << i

        #curr *= 2
        value //= 2
    if v == 0:
        del state[addr]
    else:
        state[addr] = v


def part1(input: List[str]) -> int:
    state = {}
    mask_m = 0
    mask_v = 0
    mask = ""

    for line in input:
        updated, value = line.split(' = ')
        if updated == 'mask':
            mask = value
            mask_m = 0
            mask_v = 0
            for ch in value:
                mask_m *= 2
                mask_v *= 2
                if ch == '1':
                    mask_m += 1
                    mask_v += 1
                elif ch == '0':
                    mask_m += 1
        elif updated.startswith('mem'):
            addr = int(updated[4:-1])
            val = int(value)
            #existing = 0 if not addr in state else state[addr]

            apply_mask(state, addr, mask, val)
            if state[addr] == 0:
                del state[addr]

    return sum(state.values())

# def part2(input: List[str]) -> int:
