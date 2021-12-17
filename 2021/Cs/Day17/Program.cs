using System.Text.RegularExpressions;

var (start, end) = ParseInput();
var result = 0;
var count = 0;
// Find dxs that fit the x range
for (var dx = 1; dx <= end.X; dx++)
{
    var xPositions = GetPositionsFromInitialSpeed(dx).ToArray();
    if (!xPositions.Any(x => x >= start.X && x <= end.X)) continue;

    // For each dx, find dys that reach the range
    for (var dy = start.Y; dy <= 1000; dy++)
    {
        var positions = GetPositionsFromInitial2DSpeed(dx, dy).ToArray();
        if (positions.Any(p => p.X >= start.X && p.X <= end.X && p.Y >= start.Y && p.Y <= end.Y))
        {
            count++;
            var maxY = positions.Max(p => p.Y);
            result = Math.Max(result, maxY);
        }
    }
}


Console.WriteLine("Part 1: {0}", result);
Console.WriteLine("Part 2: {0}", count);

IEnumerable<int> GetPositionsFromInitialSpeed(int dx)
{
    var x = 0;
    while (x < end.X)
    {
        x += dx;
        yield return x;

        dx--;
        if (dx == 0) break;
    }
}
IEnumerable<Pos> GetPositionsFromInitial2DSpeed(int dx, int dy)
{
    var x = 0;
    var y = 0;
    while (x <= end.X && y >= start.Y)
    {
        x += dx;
        y += dy;
        yield return new(x,y);

        if (dx > 0) dx--;
        dy--;
    }
}

(Pos start, Pos end) ParseInput()
{
    var input = File.ReadAllText(args[0]).Trim();
    var match = Regex.Match(input, @"target area: x=([-\d]+)\.\.([-\d]+), y=([-\d]+)\.\.([-\d]+)");
    if (!match.Success) throw new Exception($"Invalid input: {input}");
    var x1 = int.Parse(match.Groups[1].Value);
    var x2 = int.Parse(match.Groups[2].Value);
    var y1 = int.Parse(match.Groups[3].Value);
    var y2 = int.Parse(match.Groups[4].Value);
    return (new Pos(x1,y1), new Pos(x2,y2));
}

record Pos(int X, int Y);
