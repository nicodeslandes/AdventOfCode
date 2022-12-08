Console.WriteLine("Part1: {0}", Part1());
Console.WriteLine("Part2: {0}", Part2());


int Part1()
{
    var grid = ReadInput();

    var N = grid.Length;
    var visibleCount = 4 * N - 4;
    for (int y = 1; y < N - 1; y++)
    {
        for (int x = 1; x < N - 1; x++)
        {
            if (IsVisible(x, y)) visibleCount++;
        }
    }

    bool IsVisible(int x, int y)
    {
        var treeHeight = grid[y][x];
        var visible = Enumerable.Range(0, y).All(yy => grid[yy][x] < treeHeight)
            || Enumerable.Range(y + 1, N - y - 1).All(yy => grid[yy][x] < treeHeight)
            || Enumerable.Range(0, x).All(xx => grid[y][xx] < treeHeight)
            || Enumerable.Range(x + 1, N - x - 1).All(xx => grid[y][xx] < treeHeight);
        //Console.WriteLine("{0},{1} ({2}) is {3}visible", x, y,  treeHeight, visible ? "" : "not ");
        return visible;
    }

    return visibleCount;
}

int Part2()
{
    var grid = ReadInput();
    var N = grid.Length;

    int DirectionScenicScore(int x, int y, int dx, int dy)
    {
        int score = 0;
        var treeHeight = grid[y][x];
        while (x > 0 && x < N - 1 && y > 0 && y < N - 1)
        {
            x += dx;
            y += dy;
            if (treeHeight > grid[y][x])
            {
                score++;
            }
            else
            {
                score++;
                break;
            }
        }

        return score;
    }
    int ScenicScore(int x, int y)
    {
        var score = DirectionScenicScore(x, y, 1, 0)
            * DirectionScenicScore(x, y, -1, 0)
            * DirectionScenicScore(x, y, 0, 1)
            * DirectionScenicScore(x, y, 0, -1);

        //Console.WriteLine("Score for {0},{1} ({2}) is {3}", x, y, grid[y][x], score);
        return score;
    }

    return Enumerable.Range(0,N)
        .SelectMany(y => Enumerable.Range(0,N).Select(x => (x,y)))
        .Max(t => ScenicScore(t.x, t.y));
}

int[][] ReadInput()
{
    return System.IO.File.ReadLines(args[0])
        .Select(l => l.Select(ch => int.Parse(ch.ToString())).ToArray())
        .ToArray();
}