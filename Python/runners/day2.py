from logging import debug
from typing import List, Optional

Memory = List[int]
Result = int


def load_memory_from_input(input: List[str]) -> Memory:
    return [int(x) for line in input for x in line.split(',')]


class Computer:
    def __init__(self, memory: Memory):
        self.memory = memory
        self.pc = 0

    def run(self) -> Result:
        op_code = self.read_op_code()
        if op_code == 1:
            a = self.read_argument()
            b = self.read_argument()
            dest = self.read_argument()
            debug("[%d](%d) <- [%d](%d) + [%d](%d)", dest,
                  self.memory[dest], a, self.memory[a], b, self.memory[b])
            self.memory[dest] = self.memory[a] + self.memory[b]

        if op_code == 2:
            a = self.read_argument()
            b = self.read_argument()
            dest = self.read_argument()
            debug("[%d](%d) <- [%d](%d) * [%d](%d)", dest,
                  self.memory[dest], a, self.memory[a], b, self.memory[b])
            self.memory[dest] = self.memory[a] * self.memory[b]

        return op_code

    def read_op_code(self):
        op_code = self.memory[self.pc]
        debug("Read op_code %d", op_code)
        self.pc += 1
        return op_code

    def read_argument(self):
        arg = self.memory[self.pc]
        self.pc += 1
        return arg


def calc(input: List[str], noun: Optional[int], verb: Optional[int]):
    memory = load_memory_from_input(input)
    computer = Computer(memory)

    # Set input
    if noun: memory[1] = noun
    if verb: memory[2] = verb

    while computer.run() != 99:
        pass

    return memory[0]


def part1(input: List[str], is_test: bool) -> int:
    return calc(input, None if is_test else 12, None if is_test else None)


def part2(input: List[str]) -> int:
    for noun in range(100):
        for verb in range(100):
            if calc(input, noun, verb) == 19690720:
                return 100 * noun + verb

    raise Exception("Unable to find result")
