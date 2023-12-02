using Humanizer;
using System.Collections.Immutable;
using System.Linq;
using System.Reflection.Metadata;
using System.Text.RegularExpressions;
using System.Threading.Channels;

Console.WriteLine("Part1: {0}", Part1());
Console.WriteLine("Part2: {0}", Part2());

int Part1()
{
    var input = ReadInput();
    return input
        .Where(g => g.Draws.All(d =>
            d.Cubes.Values.All(cs => cs switch
            {
                ("red", var count) => count <= 12,
                ("green", var count) => count <= 13,
                ("blue", var count) => count <= 14,
                var x => throw new NotSupportedException($"Wrong colour: {x.Colour}"),
            }
        )))
        .Sum(d => d.Id);
}

int Part2()
{
    var input = ReadInput();
    return input
        .Select(g => g.Draws.Aggregate(ImmutableDictionary.Create<string, int>(), (counts, d) =>
            d.Cubes.Values.Aggregate(counts, (colourCounts, c) => colourCounts.TryGetValue(c.Colour, out var count) && count > c.Count ? colourCounts : colourCounts.SetItem(c.Colour, c.Count))
        ))
        .Sum(counts => counts.Values.Multiply());
}

IEnumerable<Game> ReadInput()
{
    return File.ReadAllLines(args[0])
        .Select(l => l.Split(": ") switch
        {
            [var g, var cs] => new Game(int.Parse(g.Split(" ")[1]), cs.Split("; ").Select(x => new Draw(
                x.Split(", ").Select(x =>
                x.Split(" ") switch
                {
                    [var c, var colour] => new CubeSet(colour, int.Parse(c)),
                    var o => throw new NotSupportedException(o.ToString()),
                }).ToDictionary(cs => cs.Colour))).ToArray()),
            var o => throw new NotSupportedException(o.ToString()),
        });
}

record Game(int Id, Draw[] Draws);
record Draw(Dictionary<string, CubeSet> Cubes);
record CubeSet(string Colour, int Count);
