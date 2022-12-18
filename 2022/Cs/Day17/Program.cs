var displayGrid = false;

RunAndMeasureTime("Part1", Part1);
RunAndMeasureTime("Part2", Part2);

int Part1()
{
    var moves = ReadInput();

    return Run(moves);
}

int Part2()
{
    return 0;
}

int Run(int[] moves)
{
    var pieces = LoadPieces().ToArray();
    var topPieceY = 0;
    var i = 0;

    var topPrint = 0;
    var grid = new HashSet<Pos>();
    for (int pieceIndex = 0; pieceIndex < 2022; pieceIndex++)
    {
        // Place the piece on the grid
        var currentPiece = pieces[pieceIndex % pieces.Length].CopyAt(new(2, topPieceY + 3));
        PrintGrid();
        while (true)
        {
            var dp = new Pos(moves[(i++) % moves.Length], 0);
            // Try to move the piece
            if (currentPiece.positions.All(p =>
            {
                var np = p + dp;
                return np.X >= 0 && np.X < 7 && !grid.Contains(np);
            }))
            {
                currentPiece = currentPiece.MoveBy(dp);
            }

            PrintGrid();

            // Can the piece move down?
            if (currentPiece.positions.Any(p => grid.Contains(p + new Pos(0, -1)) || p.Y == 0))
            {
                // No: add the piece to the grid
                AddToGrid(currentPiece);
                PrintGrid();
                break;
            }

            // Yes it can move down
            currentPiece = currentPiece.MoveBy(new(0, -1));
            PrintGrid();
        }

        void PrintGrid()
        {
            if (!displayGrid)
                return;

            SetCursorPosition(0, 0);
            WriteLine("Top: {0}", topPieceY);
            var y = topPieceY;
            if (currentPiece is not null) y = currentPiece.positions.Max(p => p.Y);
            y = topPrint = Math.Max(y, topPrint);

            for (; y >= 0; y--)
            {
                for (int x = 0; x < 7; x++)
                {
                    Write(grid!.Contains(new (x, y)) ? '#' :
                        currentPiece?.positions.Contains(new(x,y)) ?? false ? '@' : '.');
                }
                WriteLine();
            }
        }
    }

    return topPieceY;

    void AddToGrid(Piece piece)
    {
        piece.positions.ForEach(p =>
        {
            grid.Add(p);
            topPieceY = Math.Max(topPieceY, p.Y + 1);
        });
    }
}

IEnumerable<Piece> LoadPieces()
{
    yield return Piece.Parse("""
        ####
        """);

    yield return Piece.Parse("""
        .#.
        ###
        .#.
        """);

    yield return Piece.Parse("""
        ..#
        ..#
        ###
        """);

    yield return Piece.Parse("""
        #
        #
        #
        #
        """);

    yield return Piece.Parse("""
        ##
        ##
        """);
}

int[] ReadInput()
{
    return ReadLinesFromInputFile(args)
        .First()
        .Select(ch => ch == '>' ? 1 : -1)
        .ToArray();
}

record Piece(ImmutableHashSet<Pos> positions)
{
    public static Piece Parse(string input)
    {
        var (x, y) = (0, 0);
        var positions = ImmutableHashSet<Pos>.Empty;
        var height = input.Count(ch => ch == '\n');
        foreach(var ch in input)
        {
            switch (ch)
            {
                case '#':
                    positions = positions.Add(new Pos(x, height - y));
                    goto case '.';
                case '.':
                    x++;
                    break;
                case '\n':
                    x = 0;
                    y++;
                    break;
                case '\r':
                    continue;
                default:
                    throw new Exception("What!?");
            }
        }

        return new Piece(positions);
    }

    public Piece CopyAt(Pos bottomLeft)
    {
        return new(positions.Select(p => p + bottomLeft).ToImmutableHashSet());
    }

    public Piece MoveBy(Pos movement)
    {
        return new(positions.Select(p => p + movement).ToImmutableHashSet());
    }
}

record struct Pos(int X, int Y)
{
    public static Pos operator+(Pos x, Pos y) => new(x.X + y.X, x.Y + y.Y);
}
