using System.Diagnostics;

var showGrid = true;
var sleepTime = TimeSpan.FromMilliseconds(0);
int count = 0;

//int CellPriority(Pos pos, int[][] ml) => ml[pos.Y][pos.X] * (pos.X + pos.Y); //* pos.X + pos.Y * pos.Y;
int CellPriority(Pos pos, int[][] ml) => ml[pos.Y][pos.X];

if (showGrid) Console.Clear();
var currentGridDisplay = new Dictionary<Pos, (int val, bool highlighted)>();

var (grid, start, end) = ParseLines();
var sw = Stopwatch.StartNew();
var part1 = Solve(grid, start, end);
Console.WriteLine("Part 1: {0} ({1:N0} ms)", part1, sw.ElapsedMilliseconds);

//grid = ExpandGrid(grid, 5);
//var part2 = Solve(grid);
//Console.WriteLine("Part 2: {0} ({1:N0} ms)", part2, sw.ElapsedMilliseconds);

//int[][] ExpandGrid(int[][] grid, int factor)
//{
//    var N = grid.Length;
//    var newGrid = Enumerable.Range(0, N * factor).Select(_ => new int[N * factor]).ToArray();
//    for (int x = 0; x < factor; x++)
//    {
//        for (int y = 0; y < factor; y++)
//        {
//            CopyInitialGrid(x * N, y * N, x + y);
//            DisplayGrid(newGrid);
//        }
//    }

//    void CopyInitialGrid(int xDest, int yDest, int increase)
//    {
//        for (int x = 0; x < N; x++)
//        {
//            for (int y = 0; y < N; y++)
//            {
//                newGrid[yDest + y][xDest + x] = (grid[y][x] + increase - 1) % 9 + 1;
//            }
//        }
//    }

//    return newGrid;

//}

int Solve(int[][] grid, Pos start, Pos end)
{
    var Y = grid.Length;
    var X = grid.Length;

    var min_lengths = grid.Select(row => Enumerable.Range(0, row.Length).Select(_ => -1).ToArray()).ToArray();
    min_lengths[start.Y][start.X] = 0;
    var cellsToCheck = new PriorityQueue<Pos, int>();
    cellsToCheck.Enqueue(start, 0);
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
            
            // If no min path len found, skip
            if (pathLen == -1) continue;
            if (min_lengths[cell.Y][cell.X] == -1 || pathLen < min_lengths[cell.Y][cell.X])
            {
                min_lengths[cell.Y][cell.X] = pathLen;
                DisplayGrid(min_lengths, cellHash);

                cellsToCheck.Enqueue(cell, CellPriority(cell, min_lengths));
                if (showGrid) cellHash.Add(cell);
            }
        }
    }

    return min_lengths[end.Y][end.X];

    int GetMinPathLen(Pos pos)
    {
        var minAdjacentLength = GetAdjacentCells(pos)
            .Where(cell => grid[pos.Y][pos.X] <= grid[cell.Y][cell.X] + 1)
            .Where(HasMinLength)
            .Select(c => min_lengths[c.Y][c.X])
            .Concat(new[] { int.MaxValue })
            .Min();

        // If no adjacent cell has any path length yet, return -1: no min length
        if (minAdjacentLength == int.MaxValue) return -1;

        return minAdjacentLength + 1;
    }

    bool HasMinLength(Pos pos) => min_lengths[pos.Y][pos.X] != -1;

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
                if (y < 0 || y >= grid.Length) continue;
                yield return new Pos(x, y);
            }
        }
    }
}

void DisplayGrid(int[][] g, ISet<Pos>? highlights = null)
{
    //if (count++ % 1000 != 0) return;
    if (!showGrid) return;

    var Y = g.Length;
    var X = g[0].Length;

    if (currentGridDisplay.Count < Math.Min(Y, Console.BufferHeight - 1) * Math.Min(X, Console.BufferWidth - 1))
    {
        for (int y = 0; y < Y; y++)
        {
            if (y >= Console.BufferHeight - 1) break;
            for (int x = 0; x < X; x++)
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
    for (int y = 0; y < Y; y++)
    {
        if (y >= bufferHeight - 1) break;
        for (int x = 0; x < X; x++)
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

(int[][] grid, Pos start, Pos end) ParseLines()
{
    var grid = Utils.ReadLinesFromInputFile(args)
        .Select(line => line.Select(ch => ch switch
        {
            'S' => -1,
            'E' => -2,
            _ => ch - 'a',
        })
        .ToArray())
    .ToArray();

    Pos? start = null;
    Pos? end = null;
    for (int y = 0; y < grid.Length; y++)
    {
        for (int x = 0; x < grid[0].Length; x++)
        {
            if (grid[y][x] == -1) start = new(x, y);
            if (grid[y][x] == -2) end = new(x, y);
            if (start != null && end != null) break;
        }
    }

    grid[start.Value.Y][start.Value.X] = 0;
    grid[end.Value.Y][end.Value.X] = 25;
    return (grid, start.Value, end.Value);
}

record struct Pos(int X, int Y);