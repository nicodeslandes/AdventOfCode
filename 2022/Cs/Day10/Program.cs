Console.WriteLine("Part1: {0}", Part1());
Console.WriteLine("Part2: {0}", Part2());


int Part1()
{
    var instructions = ReadInput().ToArray();

    var cycle = 0;
    var result = 0;
    var x = 1;

    void NextCycle()
    {
        cycle++;
        if (cycle % 40 == 20)
        {
            result += cycle * x;
        }
    }

    foreach (var instruction in instructions)
    {
        NextCycle();
        switch (instruction)
        {
            case Instruction(OpCode.NoOp, _):
                break;
            case Instruction(OpCode.AddX, int arg):
                NextCycle();
                x += arg;
                break;
            default:
                break;
        }
    }
    return result;
}

int Part2()
{
    var instructions = ReadInput().ToArray();

    var cycle = 0;
    var result = 0;
    var x = 1;

    void NextCycle()
    {
        var crtPosition = cycle % 40;
        Console.Write(crtPosition >= x - 1 && crtPosition <= x + 1 ? '#' : '.');

        cycle++;

        if (crtPosition == 39)
        {
            Console.WriteLine();
        }
    }

    foreach (var instruction in instructions)
    {
        NextCycle();
        switch (instruction)
        {
            case Instruction(OpCode.NoOp, _):
                break;
            case Instruction(OpCode.AddX, int arg):
                NextCycle();
                x += arg;
                break;
            default:
                break;
        }
    }
    return result;
}

IEnumerable<Instruction> ReadInput()
{
    return File.ReadLines(args[0])
        .Select(l => l switch
        {
            "noop" => new Instruction(OpCode.NoOp),
            var inst => new Instruction(OpCode.AddX, int.Parse(inst.Split(' ')[1])),
        });
}

enum OpCode
{
    NoOp,
    AddX,
}

record struct Instruction(OpCode OpCode, int? Parameter = null);
