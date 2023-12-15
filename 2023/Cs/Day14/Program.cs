using Move = (int dx, int dy);


Console.WriteLine("Part1: {0}", Part1());
Console.WriteLine("Part2: {0}", Part2());

long Part1()
{
    var grid = new Grid();
    grid.ReadInput();

    var total = rocks
        .GroupBy(r => r.Pos.X)
        .OrderBy(g => g.Key)
        .Select(column =>
        {
            var topSquareRock = 0;
            var sum = 0L;
            var nbPiledRocks = 0;
            foreach (var r in column.OrderBy(x => x.Pos.Y))
            {
                if (r.Type == RockType.Square)
                {
                    nbPiledRocks = 0;
                    topSquareRock = r.Pos.Y + 1;
                }
                else
                {
                    sum += Y - nbPiledRocks - topSquareRock;
                    nbPiledRocks++;
                }
            }

            Console.WriteLine("Sum for column {0}: {1}", column.Key, sum);
            return sum;
        })
        .Sum();

    return total;
}

long Part2()
{
    return 0;
}

class Grid
{
    Rock?[,] _grid = null!;

    public int Rows => _grid.GetLength(0);
    public int Cols => _grid.GetLength(1);

    public void ReadInput()
    {
        int y = 0;
        int x = 0;

        IEnumerable<Rock> GetRocks()
        {
            foreach (var line in Utils.ReadLinesFromInputFile(args))
            {
                x = line.Length;
                for (int x = 0; x < line.Length; x++)
                {
                    if (line[x] is '#' or 'O')
                    {
                        yield return new Rock(new(x, y), line[x] == '#' ? RockType.Square : RockType.Round);
                    }
                }
                y++;
            }
        }


        var rocks = GetRocks().ToArray();
        var grid = new Rock[x, y];
        foreach (var item in rocks)
        {
            grid[item.Pos.X, item.Pos.Y] = item;
        }

        _grid = grid;
    }

    void MoveRocks(Direction direction)
    {
        var position = direction switch
        {
            Direction.North => new Position(0, 0),
            Direction.South => new Position(0, Rows - 1),
            Direction.East => new Position(Cols - 1, 0),
            Direction.West => new Position(0, 0),
            _ => throw new InvalidOperationException(),
        };

        Move move1, move2;
        (move1, move2) = direction switch
        {
            Direction.North => ((dx: 0, dy: 1), (dx: 1, dy: 0)),
            Direction.South => ((0, -1), (1, 0)),
            Direction.East => ((-1, 0), (0, 1)),
            Direction.West => ((1, 0), (0, 1)),
            _ => throw new InvalidOperationException(),
        };

        for(;position.X < Cols && position.Y < Rows; position.X += move2.dx, position.Y += move2.dy)
        {
            var topAvailableIndex = 0;
            var original = (position.X, position.Y);

            for (; position.X < Cols && position.Y < Rows; position.X += move1.dx, position.Y += move1.dy)
            {
                if (_grid[position.X, position.Y] is { Type: RockType.Round } rock)
                {
                    _grid[position.X, position.Y] = null;
                    _grid[topAvailableIndex++, col] = rock;
                }
                else if (_grid[position.X, position.Y] is { Type: RockType.Square } square)
                {
                    topAvailableIndex = row + 1;
                }
            }

            position.X = original.X;
            position.Y = original.Y;

        }
    }

}

enum Direction
{
    North,
    East,
    South,
    West,
}

class State(IEnumerable<Rock> roundRocks, int score)
{
    Position[] _rocks = roundRocks.Select(r => r.Pos).ToArray();
    int Score { get; } = score;
}

record Rock(Position Pos, RockType Type);
enum RockType
{
    Square,
    Round
}