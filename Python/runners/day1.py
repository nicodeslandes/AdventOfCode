from logging import debug

def part1(input, **kargs):
    sum = 0
    for line in input:
        debug("Calculating the fuel for mass %s", line)
        if line != '':
            sum += calculate_module_fuel(int(line))
    return sum


def part2(input, **kargs):
    sum = 0
    for line in input:
        if line == '':
            break

        debug("Calculating the fuel for mass %s", line)
        mass = int(line)
        while (mass := calculate_module_fuel(mass)) > 0:
            debug("Adding mass %d", mass)
            sum += mass
    return sum


def calculate_module_fuel(mass):
    return mass // 3 - 2
