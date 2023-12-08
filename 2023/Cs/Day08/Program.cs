Console.WriteLine("Part1: {0}", Part1());
Console.WriteLine("Part2: {0}", Part2());

long Part1()
{
    var (moves, nodes) = ReadInput();
    return GetCountFromStartingPoint("AAA");

    int GetCountFromStartingPoint(string start)
    {
        int count = 0;
        var current = start;
        while (current != "ZZZ")
        {
            var choices = nodes[current];
            current = moves[count % moves.Length] == 0 ? choices.left : choices.right;
            count++;
        }

        return count;
    }
}

long Part2()
{
    var (moves, nodes) = ReadInput();
    var indexes = nodes.Keys.Where(k => k[^1] == 'A').Select(GetCycleLengthAndTerminalIndex).ToArray();

    // if n==p, we're in a special case, where the indexes ar simply k*n, k >= 1
    // so the result is simply lcm of all the ns
    var lcm = Utils.Lcm(indexes.Select(x => (long)x.n));
    
    return lcm;


    (int n, int p) GetCycleLengthAndTerminalIndex(string start)
    {
        // The nodes we go through look like this (Z denotes a terminal node)
        //
        // <------------n------------>                  
        // A --------X --------------Z
        //     m     ↑       p       ↓     p: count to go from X to Z + Z to X
        //            ----------------
        
        // So first terminal index in n, and then the cycle repeats every p iterations
        // Giving us a Z node every n + k*p

        var states = new Dictionary<(string position, int stepIndex), int>();
        var terminalPositionIndexes = new List<int>();
        int count = 0;
        var current = start;
        while (!states.ContainsKey((current, count % moves.Length)))
        {
            states[(current, count % moves.Length)] = count;
            if (current[^1] == 'Z') terminalPositionIndexes.Add(count);

            var choices = nodes[current];
            current = moves[count % moves.Length] == 0 ? choices.left : choices.right;
            count++;
        }

        var n = terminalPositionIndexes.Single();
        // m is where we just came back to
        var m = states[(current, count % moves.Length)];
        // here count = n + p - (n-m) = m+p
        var p = count - m;

        return (n, p);
    }
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