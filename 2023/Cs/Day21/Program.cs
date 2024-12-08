using Common;

Console.WriteLine("Part1: {0}", Part1());
Console.WriteLine("Part2: {0}", Part2());

long Part1()
{
    // #.#23.#
    // .3#1#..
    // #212###
    // #32#...
    // ..3..##
    // 1 => 3 (if 1 adj to 2)
    // * All 1-cell aj to a 2-cell is also a 3-cell
    // * Note: Some 1-cells can be reached by more than 1 2-cells
    // At each gen:
    //  * How many n-cell
    //  * How many reachable by (n+1)-cells: these are (n+2)cells
    //
    // Ex: 0: 1 cell (S), 1 1-reachable
    //     1: 2 cells, 2 are 2-reachable
    //     2: 3 new cells, 3 are 3-reachable; +1 from |0|
    //              Total: 4-cells, 4 are 4-reachable
    //     3: 
    var x = ReadInput();

    return 0;
}

long Part2()
{
    var x = ReadInput();

    return 0;
}

(HashSet<Position> rocks, Position start, int x, int y) ReadInput()
{
    var x = 0;
    var y = 0;
    var rocks = new HashSet<Position>();
    var start = new Position(0, 0);
    var lines = Utils.ReadLinesFromInputFile(args);
    foreach (var (row, line) in lines.Enumerate())
    {
        if (x == 0) x = line.Length;
        y = row + 1;
        foreach(var ch in line)
        {
            if (ch == '#')
                rocks.Add(new Position(x, y));
            else if (ch == 'S')
                start = new Position(x, y);
        }
    }

    return (rocks, )
}




IEnumerable<Position> FindAdjacentPositions(Position p)
{
    foreach (var d in new[] { Direction.Up, Direction.Down, Direction.Right, Direction.Left })
    {
        yield return p + ToMovement(d);
    }
}

(int dx, int dy) ToMovement(Direction direction) => direction switch
{
    Direction.Left => (-1, 0),
    Direction.Right => (1, 0),
    Direction.Up => (0, -1),
    Direction.Down => (0, 1),
    _ => throw new NotImplementedException(),
};


record Move(Direction Dir, int Length);

enum Direction
{
    Right,
    Down,
    Left,
    Up,
}

