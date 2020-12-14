from typing import List
from logging import debug


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


def apply_at_addr(state, floating_addr: List[str], value: int):
    if 'X' not in floating_addr:
        addr = 0
        for c in floating_addr:
            if c == '1':
                addr += 1
            addr *= 2

        if value == 0:
            del state[addr]
        else:
            state[addr] = value

    else:
        first_x = floating_addr.index('X')
        addr1 = list(floating_addr)
        addr1[first_x] = '0'
        addr2 = list(floating_addr)
        addr2[first_x] = '1'
        apply_at_addr(state, addr1, value)
        apply_at_addr(state, addr2, value)


def apply_mask2(state, addr, mask: str, value: int):
    new_addr = 0
    #curr = 1
    floating_addr = ""

    debug("Mask: %s", mask)
    debug("Addr: %x", addr)
    for i in range(len(mask)):
        ch = mask[-i-1]

        if ch == '0':
            floating_addr = str(addr & 1) + floating_addr
        else:
            floating_addr = ch + floating_addr
        addr //= 2

    debug("Floating addr: %s", floating_addr)
    apply_at_addr(state, list(floating_addr), value)


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

    return sum(state.values())


def part2(input: List[str]) -> int:
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

            apply_mask2(state, addr, mask, val)

    return sum(state.values())
