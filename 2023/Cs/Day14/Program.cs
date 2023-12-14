

Console.WriteLine("Part1: {0}", Part1());
Console.WriteLine("Part2: {0}", Part2());

long Part1()
{
    var (Y, rocks) = ReadInput();
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
    var input = ReadInput();
    return 0;
}

(int y, Rock[] rocks) ReadInput()
{
    int y = 0;

    IEnumerable<Rock> GetRocks()
    {
        foreach (var line in Utils.ReadLinesFromInputFile(args))
        {
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
    return (y, rocks);
}

record Rock(Position Pos, RockType Type);
enum RockType
{
    Square,
    Round
}