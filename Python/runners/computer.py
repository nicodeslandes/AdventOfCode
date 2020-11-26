from abc import ABC, abstractmethod
from logging import debug
from typing import Callable, Dict, Hashable, List, Optional

Memory = List[int]
Result = int

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
    
class InstructionProcessor(ABC):
    @abstractmethod
    def execute(self, memory: Memory, pc: int, op_code: int): pass

class Instruction3ParamProcessor(InstructionProcessor):
    def __init__(self, action: Callable[[Param, Param, Param], None]):
        self._action = action

    def execute(self, memory: Memory, pc: int, op_code: int):
        # Extract 3 parameters
        op_code //= 10
        def read_param():
            nonlocal op_code, pc
            positional_parameter = op_code % 10 == 0
            op_code //= 10
            value = memory[pc]
            pc += 1
            if positional_parameter:
                return PositionalParam(memory, value)
            else:
                return ImmediateParam(value)

        a =  read_param()
        b =  read_param()
        c =  read_param()

        self._action(a,b,c)
        return pc

op_codeRegistry:Dict[int, InstructionProcessor] = {}
op_codeRegistry[1] = Instruction3ParamProcessor(lambda x, y, d: d.set(x.get() + y.get()))
op_codeRegistry[2] = Instruction3ParamProcessor(lambda x, y, d: d.set(x.get() * y.get()))
    
class Computer:
    def __init__(self, memory: Memory):
        self.memory = memory
        self.pc = 0

    def run(self) -> Result:
        op_code = self.read_op_code()
        if op_code == 99:
            debug("Halting!")
            return op_code

        processor = op_codeRegistry.get(op_code)
        if not processor:
            raise Exception(f'Unknown opcode: {op_code}')
        self.pc = processor.execute(self.memory, self.pc, op_code)
        return op_code

    def read_op_code(self):
        op_code = self.memory[self.pc]
        debug("Read op_code %d", op_code)
        self.pc += 1
        return op_code

