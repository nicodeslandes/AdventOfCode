using System;

Console.WriteLine("Part1: {0}", Part1());
Console.WriteLine("Part2: {0}", Part2());

long Part1()
{
    var grid = ReadInput();
    return FindEnergisedCellCount(grid, new(new(0, 0), Direction.Right));
}

long Part2()
{
    var grid = ReadInput();
    IEnumerable<Beam> StartBeams()
    {
        for (int x = 0; x < grid!.GetLength(0); x++)
        {
            yield return new Beam(new(x, 0), Direction.Down);
            yield return new Beam(new(x, grid.GetLength(1) - 1), Direction.Up);
        }

        for (int y = 0; y < grid.GetLength(1); y++)
        {
            yield return new Beam(new(0, y), Direction.Right);
            yield return new Beam(new(grid.GetLength(0) - 1, y), Direction.Left);
        }
    }

    return StartBeams().Max(b => FindEnergisedCellCount(grid, b));
}

int FindEnergisedCellCount(Cell[,] grid, Beam startBeam)
{
    var gridX = grid.GetLength(0);
    var gridY = grid.GetLength(1);

    var beams = new HashSet<Beam> { startBeam };
    var activeBeams = new HashSet<Beam> { beams.First() };

    while (activeBeams.Count > 0)
    {
        var newActiveBeams = new HashSet<Beam>();
        foreach (var beam in activeBeams)
        {
            foreach (var newBeam in MoveBeam(beam))
            {
                if (beams.Add(newBeam))
                {
                    newActiveBeams.Add(newBeam);
                }
            }
        }

        activeBeams = newActiveBeams;
    }

    return beams.Select(b => b.Pos).Distinct().Count();

    IEnumerable<Beam> MoveBeam(Beam beam)
    {
        return NextCellPositions(beam.Pos, beam.Dir).Select(x => new Beam(x.pos, x.dir));
    }

    IEnumerable<(Position pos, Direction dir)> NextCellPositions(Position pos, Direction direction)
    {
        //var move = ToMovement(direction);
        //var nextPosition = pos + move;
        //if (!IsValidPosition(nextPosition)) yield break;

        //TODO: don't move beam if next cell is -/\|
        var cell = grid![pos.X, pos.Y];
        switch ((cell.Type, direction))
        {
            case ('.', _)
                or ('-', Direction.Left or Direction.Right)
                or ('|', Direction.Up or Direction.Down):
                var nextPos = pos + ToMovement(direction);
                if (IsValidPosition(nextPos))
                    yield return (nextPos, direction);
                break;
            case ('/' or '\\', var dir):
                var newDir = ChangeDirection(dir, cell.Type);
                pos = GetNextPosition(pos, newDir);
                if (!IsValidPosition(pos)) yield break;
                yield return (pos, newDir);
                break;
            case ('|', _):
                foreach (var x in NextPositions(pos, [Direction.Up, Direction.Down]))
                    yield return x;
                break;
            case ('-', _):
                foreach (var x in NextPositions(pos, [Direction.Left, Direction.Right]))
                    yield return x;
                break;

            case var x: throw new InvalidOperationException(x.ToString());
        }

        IEnumerable<(Position pos, Direction dir)> NextPositions(Position pos, IEnumerable<Direction> directions)
        {
            foreach (var dir in directions)
            {
                var next = GetNextPosition(pos, dir);
                if (IsValidPosition(next)) yield return (next, dir);
            }
        }

    }

    Position GetNextPosition(Position pos, Direction dir) => pos + ToMovement(dir);

    bool IsValidPosition(Position pos)
        => pos.X >= 0 && pos.X < gridX && pos.Y >= 0 && pos.Y < gridY;

    (int dx, int dy) ToMovement(Direction direction) => direction switch
    {
        Direction.Left => (-1, 0),
        Direction.Right => (1, 0),
        Direction.Up => (0, -1),
        Direction.Down => (0, 1),
        _ => throw new NotImplementedException(),
    };
}
Cell[,] ReadInput()
{
    int y = 0;
    int x = 0;

    IEnumerable<Cell> GetCells()
    {
        foreach (var line in Utils.ReadLinesFromInputFile(args))
        {
            x = line.Length;
            for (int x = 0; x < line.Length; x++)
            {
                yield return new Cell(new Position(x, y), line[x]);
            }
            y++;
        }
    }


    var cells = GetCells().ToArray();
    var grid = new Cell[x, y];
    foreach (var item in cells)
    {
        grid[item.Pos.X, item.Pos.Y] = item;
    }

    return grid;
}


static Direction ChangeDirection(Direction dir, char cell)
    => cell switch
    {
        '/' => dir switch
        {
            Direction.Up => Direction.Right,
            Direction.Left => Direction.Down,
            Direction.Right => Direction.Up,
            Direction.Down => Direction.Left,
            _ => throw new NotImplementedException(),
        },
        '\\' => dir switch
        {
            Direction.Up => Direction.Left,
            Direction.Left => Direction.Up,
            Direction.Right => Direction.Down,
            Direction.Down => Direction.Right,
            _ => throw new NotImplementedException(),
        },
        _ => throw new NotImplementedException()
    };

record Cell(Position Pos, char Type)
{
    HashSet<Beam> Beams { get; } = new();
}

record Beam(Position Pos, Direction Dir);

enum Direction
{
    Left,
    Right,
    Up,
    Down,
}
