Console.WriteLine("Part1: {0}", Part1());
Console.WriteLine("Part2: {0}", Part2());

// 1 - Rock
// 2 - Paper
// 3 - Scissor

int Part1()
{
    return ReadInput()
        .Select(x =>
        {
            var (first, second) = x;                
            return GetScore(first, second);
        })
        .Sum();
}

int GetScore(int first, int second) =>
    second + (
        (first == second) ? 3
        : ((second - first + 3) % 3 == 1) ? 6 : 0);

int Part2()
{
    return ReadInput()
        .Select(x =>
        {
            var (first, result) = x;
            var second = result switch
            {
                1 => (first + 1) % 3 + 1,
                2 => first,
                3 => first % 3 + 1,
                _ => throw new Exception("What!??"),
            };
            return GetScore(first, second);
        })
        .Sum();
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
