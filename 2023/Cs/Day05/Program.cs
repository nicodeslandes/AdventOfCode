using System.ComponentModel.Design;
using System.Diagnostics.CodeAnalysis;
using MapType = int;
Console.WriteLine("Part1: {0}", Part1());
Console.WriteLine("Part2: {0}", Part2());

long Part1()
{
    var (seeds, maps) = ReadInput();

    return seeds.Min(i =>
        maps.Values.Aggregate(i, (v, map) => Map(v, map)));

    long Map(long value, RangeMapDictionary rangeMaps)
    {
        return 0;
        //var mapped = rangeMaps.SelectNonNull(m => m.Map(value)).FirstOrDefault();
        //return mapped == 0 ? value : mapped;
    }
}

long Part2()
{
    var (seeds, maps) = ReadInput();
    var maxMapType = maps.Keys.Max();

    var results = maps.Keys
        .Select(mapType => KeyValuePair.Create(mapType, new Dictionary<long, RangeMap>()))
        .ToDictionary();

    return 0;
    //RangeMap MapValue(long value, MapType mapType)
    //{
    //    if (results[mapType].TryGetValue(value, out var result))
    //    {
    //        return result;
    //    }

    //    result = Map(value, mapType, maps[mapType]);
    //    if (mapType < maxMapType)
    //    {
    //        result = MapValue(result, mapType + 1);
    //    }
    //    results[mapType][value] = result;
    //    return result;
    //}

    //return seeds.Buffer(2).SelectMany(b => LongEnumerable.Range(b[0], b[1])).Min(i =>
    //    MapValue(i, 0));

    //long Map(long value, int type, RangeMapDictionary rangeMaps)
    //{
    //    if (rangeMaps.TryGetContainingRangeMap(value, out var rangeMap))
    //    {
    //        return rangeMap;
    //    }
    //    var mapped = .SelectNonNull(m => m.Map(value)).FirstOrDefault();
    //    return mapped == 0 ? value : mapped;
    //}
}

(long[] seeds, Dictionary<MapType, RangeMapDictionary> maps) ReadInput()
{
    using var fileStream = File.OpenText(args[0]) ?? throw new Exception("File open");
    var seeds = fileStream.ReadLine()?.Split(": ") switch
    {
        ["seeds", var seedValues] => seedValues.Split(" ").Select(long.Parse).ToArray(),
        var x => throw new Exception($"Invalid line: {x}"),
    };

    fileStream.ReadLine();

    var maps = Enumerable.Range(0, 7)
        .Select(i => KeyValuePair.Create(i, ReadRangeMaps()))
        .ToDictionary();

    return (seeds, maps);

    RangeMapDictionary ReadRangeMaps()
    {
        var tree = new RangeMapDictionary();
        tree.AddRange(ReadRangeMapsRaw());
        return tree;
    }

    IEnumerable<RangeMap> ReadRangeMapsRaw()
    {
        fileStream.ReadLine();
        string line;
        while (!string.IsNullOrEmpty(line = fileStream.ReadLine()))
        {
            yield return line.Split(" ").Select(long.Parse).ToArray() switch
            {
                [var ss, var ds, var l] => new RangeMap(ds, ss, l),
                var x => throw new Exception(x.ToString()),
            };
        }
    }
}

class RangeMapDictionary
{
    BinaryTree<RangeMap> _ranges = new(Comparer<RangeMap>.Create((a, b) => long.Sign(b.sourceStart - a.sourceStart)));

    public void AddRange(IEnumerable<RangeMap> ranges)
    {
        _ranges.AddRange(ranges);
    }

    public bool TryGetContainingRangeMap(long value, [MaybeNullWhen(false)] out RangeMap map)
    {
        return _ranges.TryGetMatch(m => m switch
        {
            var (start, _, count) when value < start => BinaryTree.VisitorDirection.Left,
            var (start, _, count) when value <= start + count => BinaryTree.VisitorDirection.Found,
            _ => BinaryTree.VisitorDirection.Right,
        }, out map);
    }
}

record RangeMap(long sourceStart, long destinationStart, long length)
{
    public long? Map(long src) => src < sourceStart ? src :
        src >= sourceStart && src < sourceStart + length ? destinationStart + src - sourceStart :
        null;
}