using System.Linq;
using System.Text.RegularExpressions;
using System.Threading.Channels;

Console.WriteLine("Part1: {0}", Part1());
Console.WriteLine("Part2: {0}", Part2());

int Part1()
{
    var input = ReadInput();
    var l1 = input[0].OrderBy(x => x);
    var l2 = input[1].OrderBy(x => x);

    return l1.Zip(l2).Sum(x => Math.Abs(x.Second - x.First));
}

int Part2()
{
    var input = ReadInput();
    var frequencies = input[1].GroupBy(x => x).ToDictionary(g => g.Key, g => g.Count());

    return input[0].Sum(i => i * (frequencies.TryGetValue(i, out var f) ? f : 0));
}

int[][] ReadInput()
{
    var pairs = File.ReadAllLines(args[0])
        .Select(l => l.Split(" ", StringSplitOptions.RemoveEmptyEntries).ToArray());

    var output = new List<List<int>>();
    foreach (var pair in pairs)
    {
        foreach (var (i, item) in pair.Index())
        {
            if (output.Count <= i) output.Add(new());
            output[i].Add(int.Parse(item));
        }
    }

    return output.Select(o => o.ToArray()).ToArray();
}
