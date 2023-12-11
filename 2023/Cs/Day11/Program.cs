Console.WriteLine("Part1: {0}", Part1());
Console.WriteLine("Part2: {0}", Part2());

long Part1()
{
    var input = ReadInput().ToArray();
    var xSet = input.Select(p => p.X).Distinct().OrderBy(x => x).ToArray();
    var ySet = input.Select(p => p.Y).Distinct().OrderBy(x => x).ToArray();

    // x-coordinates expand for each value between positions, excepts for elements in xSet
    // ---A--x1-----x2--x3--B
    // The expansion here result in [AB] doubling, minus the 3 x between them
    var xPrefixSums = ComputePrefixSums(xSet);
    var yPrefixSums = ComputePrefixSums(ySet);

    IEnumerable<(Position a, Position b)> GalaxyPairs()
    {
        for (int i = 0; i < input.Length; i++)
        {
            for (int j = i + 1; j < input.Length; j++)
            {
                yield return (input[i], input[j]);
            }
        }
    }

    var totalXCount = xPrefixSums[^1];
    var totalYCount = yPrefixSums[^1];
    return GalaxyPairs().Select(p =>
    {
        var xCount = Math.Abs(xPrefixSums[p.a.X] - xPrefixSums[p.b.X]);
        var yCount = Math.Abs(yPrefixSums[p.a.Y] - yPrefixSums[p.b.Y]);
        return 2 * Distance(p.a, p.b) - xCount - yCount;
    }).Sum();
}

int[] ComputePrefixSums(int[] xSet)
{
    var max = xSet[^1];
    var result = new int[max + 1];
    var xi = 0;
    var sum = 0;
    for (int i = 0; i <= max; i++)
    {
        if (i == xSet[xi])
        {
            sum++;
            xi++;
        }
        result[i] = sum;
    }

    return result;
}

int Distance(Position a, Position b) => Math.Abs(b.X - a.X) + Math.Abs(b.Y - a.Y);

long Part2()
{
    var input = ReadInput();
    return 0;
}

IEnumerable<Position> ReadInput()
{
    int y = 0;
    foreach (var line in Utils.ReadLinesFromInputFile(args))
    {
        for (int x = 0; x < line.Length; x++)
        {
            if (line[x] == '#')
            {
                yield return new Position(x, y);
            }
        }
        y++;
    }
}