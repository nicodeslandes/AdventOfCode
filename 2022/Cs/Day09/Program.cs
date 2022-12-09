Console.WriteLine("Part1: {0}", Part1());
Console.WriteLine("Part2: {0}", Part2());


int Part1()
{
    var moves = ReadInput();
    var h = new Position(0, 0);
    var t = new Position(0, 0);
    var visited = new HashSet<Position> { t };
    foreach (var move in moves)
    {
        for (int i = 0; i < move.Steps; i++)
        {
            h = h.Move(move.Direction);
            t = t.Follow(h);
            visited.Add(t);
        }
    }

    return visited.Count;
}

int Part2()
{
    var moves = ReadInput();
    var knots = Enumerable.Range(0, 10).Select(_ => new Position(0, 0)).ToArray();

    var visited = new HashSet<Position> { new(0, 0) };
    foreach (var move in moves)
    {
        for (int i = 0; i < move.Steps; i++)
        {
            knots[0] = knots[0].Move(move.Direction);
            for (int k = 1; k < knots.Length; k++)
            {
                knots[k] = knots[k].Follow(knots[k-1]);
            }
            visited.Add(knots[^1]);
        }
    }

    return visited.Count;
}

IEnumerable<Move> ReadInput()
{
    return File.ReadLines(args[0])
        .Select(l => new Move(l[0] switch
        {
            'U' => new Direction(0, 1),
            'R' => new Direction(1, 0),
            'D' => new Direction(0, -1),
            'L' => new Direction(-1, 0),
            _ => throw new Exception("??"),
        }, int.Parse(l[2..])));
}

record struct Position(int X, int Y)
{
    public Position Move(Direction dir) => new(X + dir.DeltaX, Y + dir.DeltaY);

    public static Position operator-(Position left, Position right)
    {
        return new(left.X - right.X, left.Y - right.Y);
    }

    public int MaxDistance()
    {
        return Math.Max(Math.Abs(X), Math.Abs(Y));
    }

    public Position Follow(Position h)
    {
        var d = h - this;
        if (d.MaxDistance() > 1)
        {
            var dx = Math.Abs(d.X) > 0 ? d.X / Math.Abs(d.X) : 0;
            var dy = Math.Abs(d.Y) > 0 ? d.Y / Math.Abs(d.Y) : 0;
            return Move(new(dx, dy));
        }

        return this;
    }
}

record Direction(int DeltaX, int DeltaY);

record Move(Direction Direction, int Steps);