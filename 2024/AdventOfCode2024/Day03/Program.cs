using System.Runtime.CompilerServices;
using System.Text.RegularExpressions;

Console.WriteLine("Part1: {0}", Part1());
Console.WriteLine("Part2: {0}", Part2());

int Part1()
{
    var input = ReadInput();

    return input
        .SelectMany(l => PuzzleRegex.Part1.Matches(l))
        .Select(m =>
        {
            int x = int.Parse(m.Groups[1].ValueSpan);
            int y = int.Parse(m.Groups[2].ValueSpan);
            return x * y;
        })
        .Sum();
    }

int Part2()
{
    var enabled = true;
    var input = ReadInput();
    return input
        .SelectMany(l => PuzzleRegex.Part2.Matches(l))
        .Select(m =>
        {
            if (m.Groups[3].Success)
            {
                enabled = m.Groups[3].Value == "do";
                return 0;
            }

            if (!enabled)
            {
                return 0;
            }

            int x = int.Parse(m.Groups[1].ValueSpan);
            int y = int.Parse(m.Groups[2].ValueSpan);
            return x * y;
        })
        .Sum();
}

IEnumerable<string> ReadInput()
{
    return Utils.ReadLinesFromInputFile(args);
}

static partial class PuzzleRegex
{
    [GeneratedRegex(@"mul\((\d{1,3}),(\d{1,3})\)")]
    public static partial Regex Part1 { get; }

    [GeneratedRegex(@"mul\((\d{1,3}),(\d{1,3})\)|(do(?!n)|don't)")]
    public static partial Regex Part2 { get; }
}