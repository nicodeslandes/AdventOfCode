using System.Diagnostics;

namespace Common;

public static class Utils
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

    public static long Lcm(params long[] arr) => Lcm(arr);

    public static long Lcm(IEnumerable<long> enumerable)
    {
        // a = p1.p2
        // b = p1
        // c = p3

        // lcm = p1.p2.p3
        // lcm(a,b) = p1.p2
        // lcm(a,b,c) = lcm(p1.p2, p3) = p1.p2.p3

        // lcm = q.r.s = a.b.c / p3
        return enumerable.Aggregate(Lcm);
    }

    public static long Lcm(long a, long b) => a / Gcd(a, b) * b;

    public static long Gcd(params long[] arr) => Gcd((IEnumerable<long>)arr);
    
    public static long Gcd(IEnumerable<long> enumerable)
    {
        return enumerable.Aggregate(Gcd);
    }

    public static long Gcd(long x, long y)
    {
        // x ∧ y = x ∧ (y % x)
        if (x > y)
            (x, y) = (y, x);

        while (x != 0)
        {
            y %= x;

            if (x > y)
                (x, y) = (y, x);
        }

        return y;
    }

    public static LinkedListNode<T>? FindFirst<T>(this LinkedList<T> list, Func<T, bool> predicate)
    {
        var node = list.First;
        while (node != null)
        {
            if (predicate(node.Value))
                break;
            
            node = node.Next;
        }

        return node;
    }
}

public record Position
{
    public Position(int x, int y)
    {
        X = x;
        Y = y;
    }
    public int X { get; set; }
    public int Y { get; set; }

    public static Position operator +(Position pos, (int dx, int dy) move) => new(pos.X + move.dx, pos.Y + move.dy);

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