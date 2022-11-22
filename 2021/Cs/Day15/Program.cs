var grid = ParseLines();
var N = grid.Length;
var min_lengths = grid.Select(row => new int[row.Length]).ToArray();
var cellsToCheck = new HashSet<Pos> { new Pos(0, 0) };
while (cellsToCheck.Count > 0)
{
    var current = cellsToCheck.First();
    cellsToCheck.Remove(current);

    foreach (var cell in GetAdjacentCells(current))
    {
        var pathLen = GetMinPathLen(cell);
        if (min_lengths[cell.Y][cell.X] == 0 || pathLen < min_lengths[cell.Y][cell.X])
        {
            min_lengths[cell.Y][cell.X] = pathLen;
            DisplayGrid();

            cellsToCheck.Add(cell);
        }
    }
}

Console.WriteLine("Part 1: {0}", min_lengths[N - 1][N-1]);

void DisplayGrid()
{
    //Console.Write(".");
    return;
    //Thread.Sleep(1000);
    Console.SetCursorPosition(0, 0);
    for (int y = 0; y < N; y++)
    {
        for (int x = 0; x < N; x++)
        {
            Console.SetCursorPosition(x * 3, y * 3);
            Console.Write(" {0} ", grid[y][x]);
            Console.SetCursorPosition(x * 3, y * 3 + 1);
            var min = min_lengths[y][x];
            if (min < 10) Console.Write(" {0} ", min);
            else Console.Write("{0,3}", min);
        }
        Console.WriteLine();
    }
}

int GetMinPathLen(Pos pos)
{
    var minAdjacentLength = GetAdjacentCells(pos)
        .Where(HasMinLength)
        .Select(c => min_lengths[c.Y][c.X])
        .Concat(new[] { int.MaxValue })
        .Min();

    // If no adjacent cell has any path length yet, return the cell's value
    if (minAdjacentLength == int.MaxValue) return grid[pos.Y][pos.X];

    return minAdjacentLength + grid[pos.Y][pos.X];
}

bool HasMinLength(Pos pos) => min_lengths[pos.Y][pos.X] != 0;

IEnumerable<Pos> GetAdjacentCells(Pos pos)
{
    for (int dx = -1; dx <= 1; dx++)
    {
        var x = pos.X + dx;
        if (x < 0 || x >= grid[0].Length) continue;

        for (int dy = -1; dy <= 1; dy++)
        {
            if (dx == 0 && dy == 0) continue;

            // Do not allow diagonal moves
            if (dx * dy != 0) continue;

            var y = pos.Y + dy;
            if (y < 0 || y >= grid[0].Length) continue;
            yield return new Pos(x, y);
        }
    }
}

int[][] ParseLines()
{
    return Utils.ReadLinesFromInputFile(args)
        .Select(line => line.Select(ch => int.Parse(ch.ToString())).ToArray())
    .ToArray();
}

record struct Pos(int X, int Y);