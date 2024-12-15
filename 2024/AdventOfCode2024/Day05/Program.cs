Console.WriteLine("Part1: {0}", Part1());
Console.WriteLine("Part2: {0}", Part2());

bool IsCorrect(List<int> update, HashSet<(int, int)> ordering)
{
    foreach (var (x, y) in ordering)
    {
        var ix = update.IndexOf(x);
        if (ix < 0) continue;

        var iy = update.IndexOf(y);
        if (iy < 0) continue;

        if (iy < ix)
            return false;
    }

    //Console.WriteLine("Correct update: {0}; count: {1}, mid item: {2}",
    //    update.StringJoin(),
    //    update.Count,
    //    update[update.Count / 2]
    // );

    return true;
}

List<int> Reorder(List<int> update, HashSet<(int, int)> ordering)
{
    var left = update.ToList();
    var result = new List<int>();

    while (left.Count > 0)
    {
        var found = false;
        for (var i = 0; i < left.Count; i++)
        {
            var candidate = left[i];
            if (left.Index().Where(x => x.Index != i).All(x => !ordering.Contains((x.Item, candidate))))
            {
                found = true;
                result.Add(candidate);
                left.RemoveAt(i);
                break;
            }
        }

        if (!found)
        {
            throw new Exception("Woops");
        }
    }

    return result;
}

int Part1()
{
    var (ordering, updates) = ReadInput();
    return updates
        .Where(u => IsCorrect(u, ordering))
        .Select(u => u[u.Count / 2])
        .Sum();
}

int Part2()
{
    var (ordering, updates) = ReadInput();
    return updates
        .Where(u => !IsCorrect(u, ordering))
        .Select(u => Reorder(u, ordering))
        .Select(u => u[u.Count / 2])
        .Sum();
}

(HashSet<(int, int)> ordering, List<List<int>> updates) ReadInput()
{
    var ordering = new HashSet<(int, int)>();
    var updates = new List<List<int>>();

    var orderingRead = false;

    foreach (var line in Utils.ReadLinesFromInputFile(args))
    {
        if (string.IsNullOrWhiteSpace(line))
        {
            orderingRead = true;
            continue;
        }

        if (!orderingRead)
        {
            if (line.Split('|').Select(int.Parse).ToArray() is [var x, var y])
            {
                ordering.Add((x, y));
            }
        }
        else
        {
            updates.Add(line.Split(',').Select(int.Parse).ToList());
        }
    }

    return (ordering, updates);
}
