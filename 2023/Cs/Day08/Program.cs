Console.WriteLine("Part1: {0}", Part1());
Console.WriteLine("Part2: {0}", Part2());

long Part1()
{
    var (moves, nodes)= ReadInput();
    var current = "AAA";
    int count = 0;
    while (current != "ZZZ")
    {
        var choices = nodes[current];
        current = moves[count % moves.Length] == 0 ? choices.left : choices.right;
        count++;
    }

    return count;
}

long Part2()
{
    var input = ReadInput();

    return 0;
}

(int[] moves, Dictionary<string, (string left, string right)> nodes) ReadInput()
{
    int[]? moves = null;
    Dictionary<string, (string left, string right)> nodes = new();

    foreach (var line in Utils.ReadLinesFromInputFile(args))
    {
        if (moves == null)
        {
            moves = line.Select(ch => ch == 'L' ? 0 : 1).ToArray();
            continue;
        }

        if (line == "") continue;

        switch (line.Split(" = "))
        {
            case [var from, var next]:
                nodes[from] = next[1..^1].Split(", ") switch
                {
                    [var left, var right] => (left, right),
                    _ => throw new Exception("Error 2"),
                };
                ;
                break;
            default:
                throw new Exception("Error 1");
        }
    }

    return (moves!, nodes);
}