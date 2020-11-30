from abc import ABC, abstractmethod
from logging import debug
from typing import Callable, Dict, Hashable, List, Optional, Tuple
from enum import Enum

Memory = List[int]
Result = int

class ExecutionResult(Enum):
    Continue = 1
    Halt = 2
    ReadInput = 3
    WriteOutput = 4


class MemoryLoader:
    @staticmethod
    def load_memory_from_input(input: List[str]) -> Memory:
        return [int(x) for line in input for x in line.split(',')]

class Param(ABC):
    @abstractmethod
    def get(self) -> int: pass

    @abstractmethod
    def set(self, value: int) -> None: pass

class PositionalParam(Param):
    def __init__(self, memory: Memory, index: int):
        self._memory = memory
        self._index = index

    def get(self) -> int: return self._memory[self._index]
    def set(self, value: int): self._memory[self._index] = value

class ImmediateParam(Param):
    def __init__(self, value: int):
        self._value = value

    def get(self) -> int: return self._value
    def set(self, value: int): raise Exception("Attempted to dereference an immediate parameter!")
    
class InstructionContext:
    current_input: Optional[int] = None
    current_outputs: List[int] = []
    # TODO: Hook this up! And don't overwrite it if the instruction is a jump
    pc: int

    def try_read_input(self) -> Optional[int]:
        return self.current_input

    def write_output(self, value: int):
        self.current_outputs.append(value)
        
class InstructionProcessor(ABC):
    @abstractmethod
    def execute(self, ctx: InstructionContext, memory: Memory, pc: int, param_modes: int) -> Tuple[int, ExecutionResult]: pass

class Instruction1ParamProcessor(InstructionProcessor):
    def __init__(self, action: Callable[[InstructionContext, Param], ExecutionResult]):
        self._action = action

    def execute(self, ctx: InstructionContext, memory: Memory, pc: int, param_modes: int) -> Tuple[int, ExecutionResult]:
        # Extract 1 parameter
        def read_param():
            nonlocal param_modes, pc
            positional_parameter = param_modes % 10 == 0
            param_modes //= 10
            value = memory[pc]
            pc += 1
            if positional_parameter:
                return PositionalParam(memory, value)
            else:
                return ImmediateParam(value)

        a =  read_param()
        return pc, self._action(ctx, a)

class Instruction2ParamProcessor(InstructionProcessor):
    def __init__(self, action: Callable[[InstructionContext, Param, Param], None]):
        self._action = action

    def execute(self, ctx: InstructionContext, memory: Memory, pc: int, param_modes: int) -> Tuple[int, ExecutionResult]:
        # Extract 3 parameters
        def read_param():
            nonlocal param_modes, pc
            positional_parameter = param_modes % 10 == 0
            param_modes //= 10
            value = memory[pc]
            pc += 1
            if positional_parameter:
                return PositionalParam(memory, value)
            else:
                return ImmediateParam(value)

        a =  read_param()
        b =  read_param()

        self._action(ctx, a,b)
        return pc, ExecutionResult.Continue

class Instruction3ParamProcessor(InstructionProcessor):
    def __init__(self, action: Callable[[InstructionContext, Param, Param, Param], None]):
        self._action = action

    def execute(self, ctx: InstructionContext, memory: Memory, pc: int, param_modes: int) -> Tuple[int, ExecutionResult]:
        # Extract 3 parameters
        def read_param():
            nonlocal param_modes, pc
            positional_parameter = param_modes % 10 == 0
            param_modes //= 10
            value = memory[pc]
            pc += 1
            if positional_parameter:
                return PositionalParam(memory, value)
            else:
                return ImmediateParam(value)

        a =  read_param()
        b =  read_param()
        c =  read_param()

        self._action(ctx, a,b,c)
        return pc, ExecutionResult.Continue

def processReadInputInstruction(ctx: InstructionContext, x: Param) -> ExecutionResult:
    input = ctx.try_read_input()
    if not input: return ExecutionResult.ReadInput
    x.set(input)
    ctx.current_input = None
    return ExecutionResult.Continue

    
def processWriteOutputInstruction(ctx: InstructionContext, x: Param) -> ExecutionResult:
    ctx.write_output(x.get())
    return ExecutionResult.WriteOutput

def jump_if_true(ctx: InstructionContext, x: Param, y: Param):
    if x.get() != 0:
        ctx.pc = y.get()

def jump_if_false(ctx: InstructionContext, x: Param, y: Param):
    if x.get() == 0:
        ctx.pc = y.get()

def jump_if_false(ctx: InstructionContext, x: Param, y: Param):
    if x.get() == 0:
        ctx.pc = y.get()

op_codeRegistry:Dict[int, InstructionProcessor] = {}
op_codeRegistry[1] = Instruction3ParamProcessor(lambda _, x, y, d: d.set(x.get() + y.get()))
op_codeRegistry[2] = Instruction3ParamProcessor(lambda _, x, y, d: d.set(x.get() * y.get()))
op_codeRegistry[3] = Instruction1ParamProcessor(processReadInputInstruction)
op_codeRegistry[4] = Instruction1ParamProcessor(processWriteOutputInstruction)
op_codeRegistry[5] = Instruction2ParamProcessor(jump_if_true)
op_codeRegistry[6] = Instruction2ParamProcessor(jump_if_false)
op_codeRegistry[7] = Instruction3ParamProcessor(lambda _, x, y, d: d.set(int(x.get() < y.get())))
op_codeRegistry[8] = Instruction3ParamProcessor(lambda _, x, y, d: d.set(int(x.get() == y.get())))

class Computer:
    def __init__(self, memory: Memory):
        self.memory = memory
        self.pc = 0

    def run(self, ctx: InstructionContext) -> ExecutionResult:
        op_code = self.read_op_code()
        if op_code == 99:
            debug("Halting!")
            return ExecutionResult.Halt

        processor = op_codeRegistry.get(op_code % 100)
        if not processor:
            raise Exception(f'Unknown opcode: {op_code % 100}')
        new_pc, result = processor.execute(ctx, self.memory, self.pc, op_code // 100)

        debug("Op code %d executed; next pc: %d", op_code, new_pc)
        if result != ExecutionResult.ReadInput:
            self.pc = new_pc
        else:
            self.pc -= 1

        return result

    def read_op_code(self):
        op_code = self.memory[self.pc]
        debug("Read op_code %d", op_code)
        self.pc += 1
        return op_code

