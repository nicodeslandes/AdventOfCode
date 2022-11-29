using System.Diagnostics;

var showGrid = false;
var sleepTime = TimeSpan.FromMilliseconds(0);
int count = 0;

//int CellPriority(Pos pos, int[][] ml) => ml[pos.Y][pos.X] * (pos.X + pos.Y); //* pos.X + pos.Y * pos.Y;
int CellPriority(Pos pos, int[][] ml) => ml[pos.Y][pos.X];

if (showGrid) Console.Clear();
var currentGridDisplay = new Dictionary<Pos, (int val, bool highlighted)>();

var grid = ParseLines();
var sw = Stopwatch.StartNew();
var part1 = Solve(grid);
Console.WriteLine("Part 1: {0} ({1:N0} ms)", part1, sw.ElapsedMilliseconds);

grid = ExpandGrid(grid, 5);
var part2 = Solve(grid);
Console.WriteLine("Part 2: {0} ({1:N0} ms)", part2, sw.ElapsedMilliseconds);

int[][] ExpandGrid(int[][] grid, int factor)
{
    var N = grid.Length;
    var newGrid = Enumerable.Range(0, N * factor).Select(_ => new int[N * factor]).ToArray();
    for (int x = 0; x < factor; x++)
    {
        for (int y = 0; y < factor; y++)
        {
            CopyInitialGrid(x * N, y * N, x + y);
            DisplayGrid(newGrid);
        }
    }

    void CopyInitialGrid(int xDest, int yDest, int increase)
    {
        for (int x = 0; x < N; x++)
        {
            for (int y = 0; y < N; y++)
            {
                newGrid[yDest + y][xDest + x] = (grid[y][x] + increase - 1) % 9 + 1;
            }
        }
    }

    return newGrid;

}

int Solve(int[][] grid)
{
    var N = grid.Length;

    var min_lengths = grid.Select(row => new int[row.Length]).ToArray();
    var cellsToCheck = new PriorityQueue<Pos, int>();
    cellsToCheck.Enqueue(new(0, 0), 0);
    var cellHash = new HashSet<Pos> { new(0, 0) };

    DisplayGrid(grid);
    var sw = Stopwatch.StartNew();
    while (cellsToCheck.Count > 0)
    {
        var current = cellsToCheck.Dequeue();
        if (showGrid) cellHash.Remove(current);

        foreach (var cell in GetAdjacentCells(current))
        {
            var pathLen = GetMinPathLen(cell);
            if (min_lengths[cell.Y][cell.X] == 0 || pathLen < min_lengths[cell.Y][cell.X])
            {
                min_lengths[cell.Y][cell.X] = pathLen;
                DisplayGrid(min_lengths, cellHash);

                cellsToCheck.Enqueue(cell, CellPriority(cell, min_lengths));
                if (showGrid) cellHash.Add(cell);
            }
        }
    }

    return min_lengths[N - 1][N - 1];

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
}

void DisplayGrid(int[][] g, ISet<Pos> highlights = null)
{
    if (count++ % 1000 != 0) return;
    if (!showGrid) return;

    var N = g.Length;

    if (currentGridDisplay.Count < Math.Min(N, Console.BufferHeight - 1) * Math.Min(N, Console.BufferWidth - 1))
    {
        Console.Beep();
        for (int y = 0; y < N; y++)
        {
            if (y >= Console.BufferHeight - 1) break;
            for (int x = 0; x < N; x++)
            {
                if (x >= Console.BufferWidth - 1) break;
                currentGridDisplay[new(x, y)] = (-1, false);
            }
        }
    }

    //Console.Write(".");
    //return;
    if (sleepTime > TimeSpan.Zero) Thread.Sleep(sleepTime);

    var bufferHeight = Console.BufferHeight;
    var bufferWidth = Console.BufferWidth;
    for (int y = 0; y < N; y++)
    {
        if (y >= bufferHeight - 1) break;
        for (int x = 0; x < N; x++)
        {
            if (x >= bufferWidth / 2 - 1) break;
            var pos = new Pos(x, y);
            var highlight = highlights?.Contains(pos) ?? false;
            var min = g[y][x];

            var current = currentGridDisplay.TryGetValue(pos, out var v) ? v : (-1, false);
            if (current != (min, highlight))
            {
                Console.SetCursorPosition(2 * x, y);
                if (highlight) Console.BackgroundColor = ConsoleColor.Blue;
                //Console.Write("{0}", grid[y][x] % 10);
                //Console.SetCursorPosition(x, y + 1);
                Console.Write("{0:00}", min % 100);
                //if (min < 10) Console.Write(" {0} ", min);
                //else Console.Write("{0,3}", min);
                Console.BackgroundColor = ConsoleColor.Black;
                currentGridDisplay[pos] = (min, highlight);
                Console.WriteLine();
            }
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