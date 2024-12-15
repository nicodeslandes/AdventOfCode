using System.Runtime.CompilerServices;
using System.Text.RegularExpressions;

Console.WriteLine("Part1: {0}", Part1());
Console.WriteLine("Part2: {0}", Part2());

int Count(Dictionary<Position, char> grid, int rows, int cols, int dx, int dy)
{
    const string SearchString = "XMAS";
    int CountFrom(Position start)
    {
        if (dx == 0)
        {
            if (dy > 0 && start.Y != 0) return 0;
            if (dy < 0 && start.Y != rows - 1) return 0;
        }
        else if (dy == 0)
        {
            if (dx > 0 && start.X != 0) return 0;
            if (dx < 0 && start.X != cols - 1) return 0;
        }

        var matchCount = 0;
        var pos = start;
        var charIndex = 0;
        while (grid.TryGetValue(pos, out var ch))
        {
            if (ch == SearchString[charIndex])
            {
                if (++charIndex == SearchString.Length)
                {
                    matchCount++;
                    charIndex = 0;
                }
            }
            else
            {
                // No match; start the search again
                if (charIndex != 0)
                {
                    charIndex = 0;

                    // Do not move from the current position, in case this
                    // is the start of another match
                    continue;
                }
            }

            pos += (dx, dy);
        }

        return matchCount;
    }

    IEnumerable<Position> StartPositions()
    {
        for (var row = 0; row < rows; row++)
        {
            if (row != 0 && row != rows - 1)
            {
                yield return new(0, row);
                yield return new(cols - 1, row);
            }
            else
            {
                for (int col = 0; col < cols; col++)
                {
                    yield return new(col, row);
                }
            }
        }
    }

    return StartPositions()
        .Sum(start => CountFrom(start));
}

int Part1()
{
    var (grid, rows, cols) = ReadInput();
    IEnumerable<(int dx, int dy)> Directions()
    {
        return from dx in new int[] { 1, 0, -1 }
               from dy in new int[] { 1, 0, -1 }
               where (dx, dy) != (0, 0)
               select (dx, dy);
    }
    
    return Directions()
        .Select(dir => Count(grid, rows, cols, dir.dx, dir.dy))
        .Sum();
}

int Part2()
{
    var (grid, rows, cols) = ReadInput();
    var count = 0;
    for (var x = 0; x < cols; x++)
    {
        for (var y = 0;  y < rows; y++)
        {
            if (IsMatch(new Position(x, y)))
            {
                count++;
            }
        }
    }
    return count;

    bool IsMatch(Position pos)
    {
        if (pos.X <= 0 || pos.X >= cols - 1) return false;
        if (pos.Y <= 0 || pos.Y >= rows - 1) return false;
        if (grid[pos] != 'A') return false;
        return (grid[pos + (-1, -1)], grid[pos + (1, 1)]) is ('M', 'S') or ('S', 'M')
            && (grid[pos + (-1, 1)], grid[pos + (1, -1)]) is ('M', 'S') or ('S', 'M');
    }
}

(Dictionary<Position, char> grid, int rows, int cols) ReadInput()
{
    var grid = new Dictionary<Position, char>();

    foreach (var (row, line) in Utils.ReadLinesFromInputFile(args).Index())
    {
        foreach (var (col, ch) in line.Index())
        {
            grid[new(col, row)] = ch;
        }
    }

    var rows = grid.Keys.Max(k => k.Y) + 1;
    var cols = grid.Keys.Max(k => k.X) + 1;
    return (grid, rows, cols);
}
