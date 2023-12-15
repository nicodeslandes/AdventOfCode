using Move = (int dx, int dy);


Console.WriteLine("Part1: {0}", Part1());
Console.WriteLine("Part2: {0}", Part2());

long Part1()
{
    var input = ReadInput();


    return input
        .Select(CalculateHash)
        .Sum();
}

long Part2()
{
    return 0;
}

int CalculateHash(string instruction)
{
    return 0;
}

IEnumerable<string> ReadInput()
{
    return Utils.ReadLinesFromInputFile(args);
}