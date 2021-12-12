
using System.Collections.Immutable;

var count1 = ParseLines()
    .Where(l => l.Start.X == l.End.X || l.Start.Y == l.End.Y)
    .SelectMany(GeneratePositions)
    .Aggregate(ImmutableDictionary.Create<Pos, int>(), (acc, pos) => acc.SetItem(pos, acc.GetValueOrDefault(pos) + 1))
    .Values.Count(c => c > 1);

var count2 = ParseLines()
    .SelectMany(GeneratePositions)
    .Aggregate(ImmutableDictionary.Create<Pos, int>(), (acc, pos) => acc.SetItem(pos, acc.GetValueOrDefault(pos) + 1))
    .Values.Count(c => c > 1);

Console.WriteLine("Part 1: {0}", count1);
Console.WriteLine("Part 2: {0}", count2);



IEnumerable<Line> ParseLines()
{
    return Utils.ReadLinesFromInputFile(args)
    .Select(line =>
    {
        var startEnd = line.Split(" -> ")
        .Select(p =>
        {
            var pos = p.Split(',');
            return new Pos(int.Parse(pos[0]), int.Parse(pos[1]));
        }).ToArray();
        return new Line(startEnd[0], startEnd[1]);
    });
}

IEnumerable<Pos> GeneratePositions(Line line)
{
    var (start, end) = (line.Start, line.End);
    var x_inc = end.X.CompareTo(start.X);
    var y_inc = end.Y.CompareTo(start.Y);

    var pos = start;
    while(true)
    {
        yield return pos;

        if (pos == end) break;

        pos.X += x_inc;
        pos.Y += y_inc;
    }
}


record struct Pos(int X, int Y);
record Line(Pos Start, Pos End);
