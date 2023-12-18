using System;

Console.WriteLine("Part1: {0}", Part1());
Console.WriteLine("Part2: {0}", Part2());

long Part1()
{
    var moves = ReadInput().ToArray();
    var position = new Position(0, 0);
    var positions = new HashSet<Position> { position };
    foreach (var move in moves)
    {
        for (var i = 0; i < move.Length; i++)
        {
            position += ToMovement(move.Dir);
            positions.Add(position);
        }
    }

    position = new Position(1, 1);
    var newPositions = new HashSet<Position> { position };
    while (newPositions.Count > 0)
    {
        var discoveredPositions = new HashSet<Position>();
        foreach (var p in newPositions)
        {
            foreach (var n in FindAdjacentPositions(p))
            {
                if (positions.Add(n)) discoveredPositions.Add(n);
            }
        }
        newPositions = discoveredPositions;
    }

    IEnumerable<Position> FindAdjacentPositions(Position p)
    {
        foreach(var d in new[] { Direction.Up, Direction.Down, Direction.Right, Direction.Left })
        {
            yield return p + ToMovement(d);
        }
    }

    return positions.Count;
}

long Part2()
{
    var grid = ReadInput();
   
    return 0;
}

(int dx, int dy) ToMovement(Direction direction) => direction switch
{
    Direction.Left => (-1, 0),
    Direction.Right => (1, 0),
    Direction.Up => (0, -1),
    Direction.Down => (0, 1),
    _ => throw new NotImplementedException(),
};

IEnumerable<Move> ReadInput()
{
    foreach (var line in Utils.ReadLinesFromInputFile(args))
    {
        switch (line.Split(" "))
        {
            case [var d, var l, ..]:
                var length = int.Parse(l);
                var direction = d switch
                {
                    "R" => Direction.Right,
                    "D" => Direction.Down,
                    "L" => Direction.Left,
                    "U" => Direction.Up,
                    _ => throw new NotImplementedException(),
                };
                yield return new(direction, length);
                break;
        }
    }
}

record Move(Direction Dir, int Length);

enum Direction
{
    Left,
    Right,
    Up,
    Down,
}
