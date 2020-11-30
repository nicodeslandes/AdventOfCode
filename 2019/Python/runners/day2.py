from runners.computer import Computer, ExecutionResult, InstructionContext, Memory, MemoryLoader
from typing import List, Optional


def calc(input: List[str], noun: Optional[int], verb: Optional[int]):
    memory = MemoryLoader.load_memory_from_input(input)
    computer = Computer(memory)

    # Set input
    if noun: memory[1] = noun
    if verb: memory[2] = verb

    ctx = InstructionContext()
    while computer.run(ctx) != ExecutionResult.Halt:
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
