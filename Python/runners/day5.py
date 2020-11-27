from logging import info, lastResort
from runners.computer import Computer, ExecutionResult, InstructionContext, MemoryLoader
from typing import List

def part1(input: List[str], is_test: bool) -> int:
    memory = MemoryLoader.load_memory_from_input(input)
    computer = Computer(memory)
    ctx = InstructionContext()
    last_output = 0
    if is_test:
        computer.run(ctx)
        return memory[4]
    
    while True:
        result = computer.run(ctx)
        if result == ExecutionResult.ReadInput:
            ctx.current_input = 1
        elif result == ExecutionResult.WriteOutput:
            info("Output: %s", ctx.current_outputs)
            last_output = ctx.current_outputs[-1]
            ctx.current_outputs.clear()
        if result == ExecutionResult.Halt:
            break

    return last_output

# def part2(input: List[str]) -> int:
