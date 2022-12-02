Console.WriteLine("Part1: {0}", Part1());
//Console.WriteLine("Part2: {0}", Part2());

// 1 - Rock
// 2 - Paper
// 3 - Scissor

int Part1()
{
    return ReadInput()
        .Select(x =>
        {
            var (first, second) = x;
            var score = second + (
                (first == second) ? 3
                : ((second - first + 3) % 3 == 1) ? 6 : 0);
            return score;
        })
        .Sum();
}

int Part2()
{
    return 0;
}

IEnumerable<(int, int)> ReadInput()
{
    var current = new List<int>();
    foreach (var line in File.ReadLines(args[0]))
    {
        var first = line[0] - 'A' + 1;
        var second = line[2] - 'X' + 1;
        yield return(first, second);
    }
}
