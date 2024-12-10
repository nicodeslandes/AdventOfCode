using Common;
using System.Linq;
using System.Text.RegularExpressions;
using System.Threading.Channels;

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

int Part1()
{
    var input = ReadInput();
    return input.Count(IsSafe);
}

int Part2()
{
    var input = ReadInput();

    return 0;
}

int[][] ReadInput()
{
    return File.ReadAllLines(args[0])
        .Select(l => l.Split(" ", StringSplitOptions.RemoveEmptyEntries).Select(int.Parse).ToArray())
        .ToArray();
}
