using Common;
Console.WriteLine("Part1: {0}", Part1());
Console.WriteLine("Part2: {0}", Part2());

bool IsSafe(int[] levels)
{
    var sign = Math.Sign(levels[1] - levels[0]);
    if (sign == 0) return false;
    var safe = levels.GroupWithNext()
        .All(x => sign * (x.next - x.current) is > 0 and <= 3);

    //Console.WriteLine("Input: {0} - {1}", levels.StringJoin(), safe ? "safe": "unsafe");
    return safe;
}

bool IsSafe2(int[] levels)
{
    bool Safe(int skipIndex)
    {
        IEnumerable<int> levels2 = levels;
        if (skipIndex != -1)
        {
            levels2 = levels2.Index().Where(x => x.Index != skipIndex).Select(x => x.Item);
        }

        var sign = 0;

        foreach (var x in levels2.Diff())
        {
            if (sign == 0)
            {
                if (x == 0) return false;
                sign = Math.Sign(x);
            }

            if (sign * x is not (> 0 and <= 3))
                return false;
        }

        //Console.WriteLine("{0}  (diff {1}) is safe when removing value {2} at index {3}",
        //    levels.StringJoin(), levels2.Diff().StringJoin(), skipIndex == -1 ? "" : levels[skipIndex].ToString(), skipIndex);

        return true;
    }

    var result = Enumerable.Range(-1, levels.Length + 1)
        .Any(skipIndex => Safe(skipIndex));

    //Console.WriteLine("Input: {0} - {1}", levels.StringJoin(), result ? "safe" : "unsafe");
    return result;

}

int Part1()
{
    var input = ReadInput();
    return input.Count(IsSafe);
}

int Part2()
{
    var input = ReadInput();
    return input.Count(IsSafe2);
}

int[][] ReadInput()
{
    return File.ReadAllLines(args[0])
        .Select(l => l.Split(" ", StringSplitOptions.RemoveEmptyEntries).Select(int.Parse).ToArray())
        .ToArray();
}
