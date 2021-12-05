using var f = Utils.OpenInputFileAsStream(args);
var numbers = (await f.ReadLineAsync())!.Split(',').Select(x => int.Parse(x)).ToList();
Console.WriteLine("Numbers: {0}", string.Join(",", numbers));

IEnumerable<string> GetLines()
{
    string? line;
    while ((line = f!.ReadLine()) != null)
    {
        yield return line;
    }
}

var grids = GetLines()
    .Buffer(6)
    .Select(b => b
        .Skip(1)
        .Select(l => l.Split(' ', StringSplitOptions.RemoveEmptyEntries).Select(int.Parse).ToArray())
        )
    .Select(rows => new Grid(rows))
    .ToArray();

int PunchAllGridsUntilFirstWin()
{
    foreach (var n in numbers)
    {
        foreach (var g in grids)
        {
            var result = g.Punch(n);
            if (result != 0) return result;
        }
    }
    return 0;
}

int PunchAllGridsUntilNoGridLeft()
{
    var gridsLeft = Enumerable.Range(0, grids.Length).ToHashSet();
    foreach (var n in numbers)
    {
        foreach (var g_i in gridsLeft.ToArray())
        {
            var result = grids[g_i].Punch(n);
            if (result != 0
                && gridsLeft.Remove(g_i)
                && gridsLeft.Count == 0)
            {
                return result;
            }
        }
    }
    return 0;
}

Console.WriteLine("Part 1: {0}", PunchAllGridsUntilFirstWin());
Console.WriteLine("Part 2: {0}", PunchAllGridsUntilNoGridLeft());
