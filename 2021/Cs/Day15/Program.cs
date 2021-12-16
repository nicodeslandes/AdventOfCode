var grid = ParseLines();
var min_lengths = grid.Select(row => new int[row.Length]).ToArray();
var pos = new Pos(grid[0].Length - 1, grid.Length - 1);
bool done = false;
while (done)
{
    done = true;
    foreach (var cell in GetAdjacentCells(pos).Where(HasMinLength))
    {
        done = false;
        var pathLen = GetMinPathLen(cell);
        min_lengths[cell.Y][cell.X] = pathLen;

        // Now see if adjacent cells need re-evaluating
        foreach (var neighbour in GetAdjacentCells(cell).Where(HasMinLength))
        {
            Reevaluate(neighbour, exclude: cell);
        }
    }
}

void Reevaluate(Pos neighbour, Pos exclude)
{
    throw new NotImplementedException();
}

Console.WriteLine("Part 1: {0}", grid.Length);

int GetMinPathLen(Pos pos)
{
    return GetAdjacentCells(pos)
        .Where(HasMinLength)
        .Min(c => grid[c.Y][c.X] + min_lengths[c.Y][c.X]);
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