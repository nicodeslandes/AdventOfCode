Console.WriteLine("Part1: {0}", Part1());
Console.WriteLine("Part2: {0}", Part2());


long Part1()
{
    return ReadInput()
        .Where(t => IsSolvable(t.result, t.operands))
        .Sum(x => x.result);
}

bool IsSolvable(long result, long[] operands, int maxOperator = 1)
{
    IEnumerable<int[]> Operators()
    {
        var operators = new int[operands.Length - 1];
        Array.Fill(operators, '+');

        for (int i = 0; i < 1 << operators.Length; i++)
        {
            yield return operators;
            var carry = 1;
            for (int index = 0; index < operators.Length; index++)
            {
                var current = operators[index];
                if (current < maxOperator && carry == 1 || carry == 0)
                {
                    if (carry == 1)
                        operators[index]++;
                    break;
                }
                else
                {
                    // current == maxOperator && carry == 1
                    operators[index] = 0;
                }
            }
        }
    }

    long Compute(long[] operands, int[] operators)
    {
        var result = operands[0];
        for (int i = 1; i < operands.Length; i++)
        {
            result = operators[i - 1] == 0 ? result + operands[i] : result * operands[i];
        }

        return result;
    }

    return Operators().Any(ops => Compute(operands, ops) == result);
}

long Part2()
{
    ReadInput();
    return 0;
}

IEnumerable<(long result, long[] operands)> ReadInput()
{
    foreach (var line in Utils.ReadLinesFromInputFile(args))
    {
        if (line.Split(':') is [var result, var operands])
        {
            yield return (long.Parse(result), operands.Split(' ', StringSplitOptions.RemoveEmptyEntries).Select(long.Parse).ToArray());
        }
    }
}

record Cursor(Position Pos, Position Dir);
