
using System.Net.WebSockets;

Console.WriteLine("Part1: {0}", Part1());
Console.WriteLine("Part2: {0}", Part2());

long Part1()
{
    var input = ReadInput();
    return input.Sum(PredictNextValue);
}

long PredictNextValue(long[] values)
{
    var sum = 0L;
    while (values.Length > 1 && values.Any(x => x != 0))
    {
        sum += values[^1];
        values = values.Zip(values.Skip(1)).Select(x => x.Second - x.First).ToArray();
    }

    return sum;
}

long Part2()
{
    var input = ReadInput();
    return input.Sum(PredictPreviousValue);
}

long PredictPreviousValue(long[] values)
{
    var alternateSum = 0L;
    int i = 1;
    while (values.Length > 1 && values.Any(x => x != 0))
    {
        alternateSum += Math.Sign(i) * values[0];
        i = -i;
        values = values.Zip(values.Skip(1)).Select(x => x.Second - x.First).ToArray();
    }

    return alternateSum;
}

IEnumerable<long[]> ReadInput()
{
    foreach (var line in Utils.ReadLinesFromInputFile(args))
    {
        yield return line.Split(' ').Select(long.Parse).ToArray();
    }
}