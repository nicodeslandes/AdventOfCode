Console.WriteLine("Part1: {0}", Part1());
Console.WriteLine("Part2: {0}", Part2());

int Part1()
{
    return ReadElvesCalories().Max(cals => cals.Sum());
}

int Part2()
{
    return ReadElvesCalories()
        .Select(cal => cal.Sum())
        .Order()
        .TakeLast(3)
        .Sum();
}

IEnumerable<int[]> ReadElvesCalories()
{
    var current = new List<int>();
    foreach (var line in File.ReadLines(args[0]))
    {
        if (line.Trim().IsEmpty())
        {
            yield return current.ToArray();
            current.Clear();
        }
        else current.Add(int.Parse(line.Trim()));
    }

    yield return current.ToArray();
}
