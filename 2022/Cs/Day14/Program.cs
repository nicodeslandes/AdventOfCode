RunAndMeasureTime("Part1", Part1);
RunAndMeasureTime("Part2", Part2);

int Part1()
{
    var grid = ReadInput();
    while (true)
    {
        var sand = new Pos(500, 0);
        if (!grid.DropSand(sand)) break;
    }

    return grid.Count(c => c == Content.Sand);
}

int Part2()
{
    return 0;
}

Grid ReadInput()
{
    var grid = new Grid();
    foreach (var line in ReadLinesFromInputFile(args))
    {
        var path = line.Split(" -> ").Select(pos =>
        {
            var xy = pos.Split(',').Select(int.Parse).ToArray();
            return new Pos(xy[0], xy[1]);
        });

        Pos? prev = null;
        foreach (var pt in path)
        {
            if (prev is not { } pos)
            {
                prev = pt;
                continue;
            }

            var (dx, dy) = (0, 0);
            if (pos.X == pt.X) dy = Math.Sign(pt.Y - pos.Y);
            else dx = Math.Sign(pt.X - pos.X);

            while (pos != pt)
            {
                grid.AddStone(pos);
                pos = new(pos.X + dx, pos.Y + dy);
            }

            grid.AddStone(pt);
            prev = pt;
        }
    }

    return grid;
}

record struct Pos(int X, int Y);

enum Content
{
    Empty,
    Sand,
    Stone
}

class Grid
{
    private Dictionary<Pos, Content> _cells = new();
    private int _bottom;

    public void AddStone(Pos pos)
    {
        _cells[pos] = Content.Stone;
        _bottom = Math.Max(_bottom, pos.Y);
    }

    public Content this[Pos pos]
    {
        get
        {
            _cells.TryGetValue(pos, out var result);
            return result;
        }

        set => _cells[pos] = value;
    }

    public bool DropSand(Pos sand)
    {
        while (true)
        {
            var emptyPos = Attempts(sand)
                .Where(p => this[p] == Content.Empty)
                .Select(p => (Pos?)p)
                .FirstOrDefault();
            if (emptyPos is Pos pos)
            {
                sand = pos;
                if (sand.Y >= _bottom)
                    return false;
            }
            else
            {
                this[sand] = Content.Sand;
                return true;
            }
        }

        IEnumerable<Pos> Attempts(Pos pos)
        {
            yield return new(pos.X, pos.Y + 1);
            yield return new(pos.X - 1, pos.Y + 1);
            yield return new(pos.X + 1, pos.Y + 1);
        }
    }

    public int Count(Func<Content, bool> predicate)
        => _cells.Values.Count(predicate);
}