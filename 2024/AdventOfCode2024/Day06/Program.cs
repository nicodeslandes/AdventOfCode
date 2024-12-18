Console.WriteLine("Part1: {0}", Part1());
Console.WriteLine("Part2: {0}", Part2());


int GetPath(char[][] inputGrid, Cursor cursor, Position? obstacle = null)
{
    HashSet<Cursor> seen = [];

    var grid = inputGrid.Select(r => r.ToArray()).ToArray();
    var gx = grid[0].Length;
    var gy = grid.Length;

    if (obstacle is var (ox, oy))
    {
        grid[oy][ox] = '#';
    }

    while (true)
    {
        var wentOut = false;
        if (!seen.Add(cursor))
        {
            return -1; // cycle detected
        }

        grid[cursor.Pos.Y][cursor.Pos.X] = 'X';
        for (int i = 0; i < 4; i++)
        {
            var next = cursor with { Pos = cursor.Pos + (cursor.Dir.X, cursor.Dir.Y) };
            if (next.Pos.X < 0 || next.Pos.X >= gx
                || next.Pos.Y < 0 || next.Pos.Y >= gy)
            {
                wentOut = true;
                break;
            }

            if (grid[next.Pos.Y][next.Pos.X] != '#')
            {
                cursor = next;
                break;
            }
            else
            {
                cursor = cursor with { Dir = cursor.Dir.RotateLeft() };
            }
        }

        if (wentOut) break;
    }

    return grid.SelectMany(r => r).Count(x => x == 'X');
}

int Part1()
{
    var (grid, cursor) = ReadInput();
    return GetPath(grid, cursor);
}

int Part2()
{
    var (grid, cursor) = ReadInput();
    var gx = grid[0].Length;
    var gy = grid.Length;

    bool IsLoop(int x, int y) => GetPath(grid, cursor, new(x, y)) == -1;

    return Enumerable.Range(0, gx)
        .SelectMany(x => Enumerable.Range(0, gy).Select(y => (x, y)))
        .Where(t => grid[t.y][t.x] == '.' && IsLoop(t.x, t.y))
        .Count();
}

(char[][] grid, Cursor cursor) ReadInput()
{    
    var grid = new List<List<char>>();
    Cursor? cursor = null;
    foreach (var line in Utils.ReadLinesFromInputFile(args))
    {
        grid.Add(line.ToList());
        var x = grid[^1].IndexOf('^');
        if (x != -1)
        {
            cursor = new Cursor(new(x, grid.Count - 1), new(0, -1));
        }
    }

    return (grid.Select(r => r.ToArray()).ToArray(), cursor ?? throw new Exception("Missing cursor"));
}

record Cursor(Position Pos, Position Dir);
