using System.Diagnostics;

namespace Common;

public class Utils
{
    public static void RunAndMeasureTime<T>(string label, Func<T> func)
    {
        var sw = Stopwatch.StartNew();
        var result = func();
        Console.WriteLine("{0}: {1} ({2:N0} ms)", label, result, sw.ElapsedMilliseconds);
    }

    public static StreamReader OpenInputFileAsStream(string[] args)
    {
        if (args.Length < 1)
        {
            Console.WriteLine("Missing file name");
            Environment.Exit(1);
        }

        return File.OpenText(args[0]) ?? throw new Exception($"Failed to open file {args[0]}");
    }

    public static IEnumerable<string> ReadLinesFromInputFile(string[] args)
    {
        using var reader = OpenInputFileAsStream(args);
        foreach (var line in ReadLines(reader))
        {
            yield return line;
        }
    }

    public static IEnumerable<string> ReadLines(StreamReader reader)
    {
        string? line;
        while ((line = reader!.ReadLine()) != null)
        {
            yield return line;
        }
    }
}

public record Position(int X, int Y)
{
    public override string ToString() => $"({X},{Y})";

    public IEnumerable<Position> AdjacentPositions(Adjacency adjacency = Adjacency.All)
    {
        if (adjacency == Adjacency.All)
        {
            for (var dy = -1; dy <= 1; dy++)
            {
                if (adjacency == Adjacency.DiagonalCross && dy == 0) continue;
                for (var dx = -1; dx <= 1; dx++)
                {
                    if (dx == 0 && dy == 0) continue;

                    if (adjacency == Adjacency.DiagonalCross && dy == 0) continue;
                    if (adjacency == Adjacency.StraightCross && dx != 0 && dy != 0) continue;
                    yield return new Position(X + dx, Y + dy);
                }
            }
        }
    }
}

public enum Adjacency
{
    All,    // Full 9x9 grid
    DiagonalCross,
    StraightCross,
}