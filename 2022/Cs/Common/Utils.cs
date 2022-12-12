using System.Diagnostics;

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
