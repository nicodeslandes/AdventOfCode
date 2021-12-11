const bool ShowGrid = true;
var grid = ParseLines();
var flashCount = 0;
int i;
for (i = 0; i < 100; i++)
{
    NextStep(grid, ref flashCount);
}


while (!AllGridFlashed(grid))
{
    NextStep(grid, ref flashCount);
    i++;
}
Console.WriteLine("Part 1: {0}", flashCount);
Console.WriteLine("Part 2: {0}", i);

bool AllGridFlashed(int[][] grid)
{
    return grid.SelectMany(row => row).All(c => c == 0);
}

int[][] ParseLines()
{
    return Utils.ReadLinesFromInputFile(args)
    .Select(line => line.Select(ch => ch - '0').ToArray())
    .ToArray();
}

void NextStep(int[][] grid, ref int flashCount)
{
    var original = grid.Select(row => row.Select(c => c).ToArray()).ToArray();

    void ForEachCell(Action<Pos> action)
    {
        for (int y = 0; y < grid.Length; y++)
        {
            for (int x = 0; x < grid[0].Length; x++)
            {
                action(new Pos(x, y));
            }
        }
    }

    var flashingCells = new HashSet<Pos>();
    void IncrementCell(Pos pos)
    {
        var cell = grid[pos.Y][pos.X];
        if (cell < 10 && ++grid[pos.Y][pos.X] == 10) flashingCells.Add(pos);
    }

    IEnumerable<Pos> GetAdjacentCells(Pos pos)
    {
        var (x, y) = pos;
        return from dx in Enumerable.Range(-1, 3)
               from dy in Enumerable.Range(-1, 3)
               where dx != 0 || dy != 0
               let nx = x + dx
               let ny = y + dy
               where nx >= 0 && nx < grid[0].Length && ny >= 0 && ny < grid.Length
               select new Pos(nx, ny);
    }

    Print(grid);
    ForEachCell(IncrementCell);
    Print(grid);
    while (flashingCells.Count > 0)
    {
        var flashes = flashingCells.ToArray();
        flashCount += flashes.Length;
        flashingCells.Clear();
        foreach (var pos in flashes)
        {
            foreach (var cell in GetAdjacentCells(pos))
            {
                IncrementCell(cell);
                Print(grid);
            }
        }

        Print(grid);
    }
    ForEachCell(p => grid[p.Y][p.X] = grid[p.Y][p.X] % 10);

}

void Print(int[][] grid)
{
    if (!ShowGrid) return;
    for (int y = 0; y < grid.Length; y++)
    {
        for (int x = 0; x < grid[0].Length; x++)
        {
            Console.Write(grid[y][x] % 10);
        }
        Console.WriteLine();
    }
    Console.WriteLine();
    Console.SetCursorPosition(0, 0);
}

record struct Pos(int X, int Y);
record Line(Pos Start, Pos End);
