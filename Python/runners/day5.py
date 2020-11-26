from runners.computer import Computer, MemoryLoader
from typing import List

def part1(input: List[str], is_test: bool) -> int:
    memory = MemoryLoader.load_memory_from_input(input)
    computer = Computer(memory)

    computer.run()

    if is_test:
        return memory[4]
    
    return 0

# def part2(input: List[str]) -> int:
