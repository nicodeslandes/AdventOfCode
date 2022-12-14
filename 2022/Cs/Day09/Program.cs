using System.Text;

TimeSpan DisplayDelay = TimeSpan.FromMilliseconds(1);

Console.OutputEncoding = Encoding.UTF8;
Console.WriteLine("Part1: {0}", Part1());
Console.WriteLine("Part2: {0}", Part2());

int Part1()
{
    var moves = ReadInput();
    var h = new Position(0, 0);
    var t = new Position(0, 0);
    var visited = new HashSet<Position> { t };
    foreach (var move in moves)
    {
        for (int i = 0; i < move.Steps; i++)
        {
            h = h.Move(move.Direction);
            t = t.Follow(h);
            visited.Add(t);
        }
    }

    return visited.Count;
}

int Part2()
{
    var moves = ReadInput();
    var knots = Enumerable.Range(0, 10).Select(_ => new Position(0, 0)).ToArray();

    Console.CursorVisible = false;
    Console.Clear();
    var displayGrid = new DisplayGrid(DisplayDelay);
    displayGrid.UpdateKnotPositions(knots);

    var visited = new HashSet<Position> { new(0, 0) };
    foreach (var move in moves)
    {
        for (int i = 0; i < move.Steps; i++)
        {
            knots[0] = knots[0].Move(move.Direction);
            for (int k = 1; k < knots.Length; k++)
            {
                knots[k] = knots[k].Follow(knots[k-1]);
            }
            visited.Add(knots[^1]);
            displayGrid.UpdateKnotPositions(knots);
        }
    }

    Console.CursorVisible = true;
    return visited.Count;
}

IEnumerable<Move> ReadInput()
{
    return File.ReadLines(args[0])
        .Select(l => new Move(l[0] switch
        {
            'U' => new Direction(0, 1),
            'R' => new Direction(1, 0),
            'D' => new Direction(0, -1),
            'L' => new Direction(-1, 0),
            _ => throw new Exception("??"),
        }, int.Parse(l[2..])));
}

class DisplayGrid
{
    private readonly TimeSpan _displayDelay;
    private Dictionary<Position, int> _pixels = new();
    private Position[] _knotPositions = new Position[0];
    private int _gridXStart = 0;
    private int _gridXEnd = 0;
    private int _gridYStart = 0;
    private int _gridYEnd = 0;

    public DisplayGrid(TimeSpan displayDelay)
    {
        _displayDelay = displayDelay;
    }

    public void UpdateKnotPositions(Position[] knots)
    {
        var invalidatedPositions = new HashSet<Position>();
        var gridLimitXStart = _gridXStart;
        var gridLimitXEnd = _gridXEnd;
        var gridLimitYStart = _gridYStart;
        var gridLimitYEnd = _gridYEnd;

        for (int knot = 0; knot < knots.Length; knot++)
        {
            // Do we have a current position for knot?
            if (knot < _knotPositions.Length)
            {
                // Has it changed?
                var currentPos = _knotPositions[knot];
                if (currentPos != knots[knot])
                {
                    // Invalidate the original position and change it
                    _pixels[currentPos] = -1;
                    invalidatedPositions.Add(currentPos);
                }
            }

            // Always invalidate/update the new position
            var newPosition = knots[knot];
            _pixels[newPosition] = knot;
            invalidatedPositions.Add(newPosition);

            gridLimitXStart = Math.Min(gridLimitXStart, newPosition.X);
            gridLimitXEnd = Math.Max(gridLimitXEnd, newPosition.X);
            gridLimitYStart = Math.Min(gridLimitYStart, newPosition.Y);
            gridLimitYEnd = Math.Max(gridLimitYEnd, newPosition.Y);
        }

        void WritePixel(int x, int y, bool updateInvalidatedPositions = true)
        {
            var pixel = _pixels.TryGetValue(new(x, y), out var p) && p != -1 ? (p % 10).ToString() : "∙";
            Console.Write(pixel);
            if (updateInvalidatedPositions)
                invalidatedPositions!.Remove(new(x, y));
        }

        // Has the Grid top left corner moved?
        if (gridLimitXStart < _gridXStart || gridLimitYStart < _gridYStart)
        {
            // Yup, need to move the current area
            var originalWidth = _gridXEnd - _gridXStart + 1;
            var originalHeight = _gridYEnd - _gridYStart + 1;
            var startXOffset = _gridXStart - gridLimitXStart;
            var startYOffset = _gridYStart - gridLimitYStart;
#pragma warning disable CA1416 // Validate platform compatibility
            Console.MoveBufferArea(0, 0, originalWidth, originalHeight, startXOffset, startYOffset);
#pragma warning restore CA1416 // Validate platform compatibility

            // Draw the empty space
            // First, the row above the shifted grid
            Console.SetCursorPosition(0, 0);
            for (int y = gridLimitYStart; y < _gridYStart; y++)
            {
                for (int x = gridLimitXStart; x <= gridLimitXEnd; x++)
                {
                    WritePixel(x, y);
                }
                Console.WriteLine();
            }

            // Then the left side
            if (gridLimitXStart < _gridXStart)
            {
                for (int y = _gridYStart; y <= gridLimitYEnd; y++)
                {
                    Console.SetCursorPosition(0, y - gridLimitYStart);
                    for (int x = gridLimitXStart; x < _gridXStart; x++)
                    {
                        WritePixel(x, y);
                    }
                }
            }
        }

        // Has the grid bottom right corner moved?
        if (gridLimitXEnd > _gridXEnd || gridLimitYEnd > _gridYEnd)
        {
            // Yes, draw the extra pixels

            // First, the right side
            if (gridLimitXEnd > _gridXEnd)
            {
                for (int y = gridLimitYStart; y <= _gridYEnd; y++)
                {
                    Console.SetCursorPosition(_gridXEnd - gridLimitXStart + 1, y - gridLimitYStart);
                    for (int x = _gridXEnd + 1; x <= gridLimitXEnd; x++)
                    {
                        WritePixel(x, y);
                    }
                }
            }

            // Then the bottom
            if (gridLimitYEnd > _gridYEnd)
            {
                Console.SetCursorPosition(0, _gridYEnd + 1 - gridLimitYStart);
                for (int y = _gridYEnd + 1; y <= gridLimitYEnd; y++)
                {
                    for (int x = gridLimitXStart; x <= gridLimitXEnd; x++)
                    {
                        WritePixel(x, y);
                    }
                    Console.WriteLine();
                }
            }
        }

        foreach (var (x, y) in invalidatedPositions)
        {
            Console.SetCursorPosition(x - gridLimitXStart, y - gridLimitYStart);
            WritePixel(x, y, false);
        }

        _gridXStart = gridLimitXStart;
        _gridXEnd = gridLimitXEnd;
        _gridYStart = gridLimitYStart;
        _gridYEnd  = gridLimitYEnd;

        if (knots.Length != _knotPositions.Length)
        {
            _knotPositions = new Position[knots.Length];
        }
        Array.Copy(knots, _knotPositions, knots.Length);

        if (_displayDelay != TimeSpan.Zero)
            Thread.Sleep(_displayDelay);
    }
}

record struct Position(int X, int Y)
{
    public Position Move(Direction dir) => new(X + dir.DeltaX, Y + dir.DeltaY);

    public static Position operator-(Position left, Position right)
    {
        return new(left.X - right.X, left.Y - right.Y);
    }

    public int MaxDistance()
    {
        return Math.Max(Math.Abs(X), Math.Abs(Y));
    }

    public Position Follow(Position h)
    {
        var d = h - this;
        if (d.MaxDistance() > 1)
        {
            var dx = Math.Abs(d.X) > 0 ? d.X / Math.Abs(d.X) : 0;
            var dy = Math.Abs(d.Y) > 0 ? d.Y / Math.Abs(d.Y) : 0;
            return Move(new(dx, dy));
        }

        return this;
    }
}

record Direction(int DeltaX, int DeltaY);

record Move(Direction Direction, int Steps);