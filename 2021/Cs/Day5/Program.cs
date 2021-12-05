
using System.Collections.Immutable;

var count = Utils.ReadLinesFromInputFile(args)
    .Select(line =>
    {
        var startEnd = line.Split(" -> ")
        .Select(p =>
        {
            var pos = p.Split(',');
            return new Pos(int.Parse(pos[0]), int.Parse(pos[1]));
        }).ToArray();
        return new Line(startEnd[0], startEnd[1]);
    })
    .Where(l => l.Start.X == l.End.X || l.Start.Y == l.End.Y)
    .SelectMany(GeneratePositions)
    .Aggregate(ImmutableDictionary.Create<Pos, int>(), (acc, pos) => acc.SetItem(pos, acc.GetValueOrDefault(pos) + 1))
    .Values.Count(c => c > 1);

Console.WriteLine("Part 1: {0}", count);

IEnumerable<Pos> GeneratePositions(Line line)
{
    if (line.Start.X == line.End.X)
    {
        var inc = line.Start.Y < line.End.Y ? 1 : -1;
        for (int y = line.Start.Y; y != line.End.Y + inc; y += inc)
        {
            yield return new Pos(line.Start.X, y);
        }
    }
    else if (line.Start.Y == line.End.Y)
    {
        var inc = line.Start.X < line.End.X ? 1 : -1;
        for (int x = line.Start.X; x != line.End.X + inc; x += inc)
        {
            yield return new Pos(x, line.Start.Y);
        }
    }
    else
    {
        throw new InvalidOperationException("Nope!");
    }
}


readonly record struct Pos(int X, int Y);
record Line(Pos Start, Pos End);
