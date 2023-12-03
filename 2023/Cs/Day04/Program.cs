Console.WriteLine("Part1: {0}", Part1());
Console.WriteLine("Part2: {0}", Part2());

int Part1()
{
    var input = ReadInput();
    return 0;
}

int Part2()
{
    return 0;
}

Dictionary<(int x, int y), char> ReadInput()
{

    return File.ReadLines(args[0])
        .Enumerate()
        .SelectMany(l => l.value.Enumerate().Select(ch => (x: ch.index, y: l.index, ch: ch.value)))
        .Where(x => x.ch != '.')
        .ToDictionary(x => (x.x, x.y), x => x.ch);
}
