Run(Part1);
Run(Part2);

int Part1()
{
    var grid = ReadInput();
    var antennas = grid.Antennas.Values.GroupBy(a => a.Char)
        .Select(a => (ch: a.Key, positions: a.Select(x => x.Pos).ToArray()));

    var antiNodes = from a in antennas
                    from pair in a.positions.GetAllPairs(PairingFlags.Unordered | PairingFlags.DistinctItems)
                    from antiNode in GetAntiNodes(grid, pair.x, pair.y)
                    select antiNode; 


    return antiNodes.Distinct().Count();
}

int Part2()
{
    var grid = ReadInput();
    var antennas = grid.Antennas.Values.GroupBy(a => a.Char)
        .Select(a => (ch: a.Key, positions: a.Select(x => x.Pos).ToArray()));

    var antiNodes = from a in antennas
                    from pair in a.positions.GetAllPairs(PairingFlags.Unordered | PairingFlags.DistinctItems)
                    from n in GetAlignedNodes(grid, pair.x, pair.y)
                    select n;

    return antiNodes.Distinct().Count();
}

IEnumerable<Position> GetAntiNodes(Grid grid, Position x, Position y)
{
    var node1 = y + (y - x);
    if (grid.IsWithinBounds(node1)) yield return node1;

    var node2 = x + (x - y);
    if (grid.IsWithinBounds(node2)) yield return node2;
}

IEnumerable<Position> GetAlignedNodes(Grid grid, Position x, Position y)
{
    var pos = x;
    var move = y - x;

    while (grid.IsWithinBounds(pos))
    {
        pos -= move;
    }

    pos += move;
    while(grid.IsWithinBounds(pos))
    {
        yield return pos;
        pos += move;
    }
}

Grid ReadInput()
{
    var results = new Dictionary<Position, Antenna>();
    int gridX = 0;
    int y = 0;
    foreach (var line in ReadLinesFromInputFile(args))
    {
        for (var x = 0; x < line.Length; x++)
        {
            if (line[x] != '.')
            {
                var pos = new Position(x, y);
                results[pos] = new Antenna(pos, line[x]);
            }
        }

        gridX = Math.Max(gridX, line.Length);
        y++;
    }

    return new Grid(gridX, y, results);
}

record Grid(int X, int Y, Dictionary<Position, Antenna> Antennas)
{
    public bool IsWithinBounds(Position pos) => pos.X >= 0 && pos.X < X && pos.Y >= 0 && pos.Y < Y;

}

record Antenna(Position Pos, char Char);