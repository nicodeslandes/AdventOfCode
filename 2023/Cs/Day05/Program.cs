using System;
using MapType = int;
using RangeMapDictionary = System.Collections.Generic.Dictionary<System.Data.MappingType, RangeMap>;
Console.WriteLine("Part1: {0}", Part1());
Console.WriteLine("Part2: {0}", Part2());

long Part1()
{
    var (seeds, maps) = ReadInput();

    return 0;
    //return seeds.Min(i =>
    //    maps.Values.Aggregate(i, (v, map) => Map(v, map)));

    //long Map(long value, RangeSet rangeMaps)
    //{
    //    return 0;
    //    //var mapped = rangeMaps.SelectNonNull(m => m.Map(value)).FirstOrDefault();
    //    //return mapped == 0 ? value : mapped;
    //}
}

long Part2()
{
    var (seeds, maps) = ReadInput();
    var maxMapType = maps.Keys.Max();

    var results = maps.Keys
        .Select(mapType => KeyValuePair.Create(mapType, new Dictionary<long, RangeMap>()))
        .ToDictionary();
    var rangeSet = maps.Values
            .Aggregate(seeds, (ranges, maps) => ApplyMaps(ranges, maps));

    return rangeSet.Min();

    RangeSet Map(RangeMap[] maps, RangeSet rangeSet)
    {
        Console.WriteLine("From: {0}; maps: [{1}]", rangeSet, maps.StringJoin());
        var mapped = maps.Select(m => rangeSet.Map(m.Map)).ToArray();
        Console.WriteLine("Mapped: {0}", mapped.StringJoin());
        var result = mapped
            .Aggregate((r1, r2) => r1.Merge(r2));
        Console.WriteLine("To  : {0}", result);
        return result;
        //var mapped = rangeMaps.SelectNonNull(m => m.Map(value)).FirstOrDefault();
        //return mapped == 0 ? value : mapped;
    }

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

RangeSet MapRange(RangeSet set, RangeMap map)
{
    // ----a---------b-----   range
    // -s------l-----------   map from [s,s+l] to [d, d+l]
    Console.WriteLine("From: {0}; map: {1}", set, map);
    var result = set.Map(r => map.Map(r));
    Console.WriteLine("To  : {0}", result); ;
    return result;
}

(RangeSet seeds, Dictionary<MapType, RangeMap[]> maps) ReadInput()
{
    using var fileStream = File.OpenText(args[0]) ?? throw new Exception("File open");
    var seeds = fileStream.ReadLine()?.Split(": ") switch
    {
        ["seeds", var seedValues] => new RangeSet(
            seedValues.Split(" ").Select(long.Parse).Buffer(2).Select(pair => new Range(pair[0], pair[1]))),
        var x => throw new Exception($"Invalid line: {x}"),
    };

    fileStream.ReadLine();

    var maps = Enumerable.Range(0, 7)
        .Select(i => KeyValuePair.Create(i, ReadRangeMaps()))
        .ToDictionary();

    return (seeds, maps);

    RangeMap[] ReadRangeMaps()
    {
        return ReadRangeMapsRaw().ToArray();
    }

    IEnumerable<RangeMap> ReadRangeMapsRaw()
    {
        fileStream.ReadLine();
        string line;
        while (!string.IsNullOrEmpty(line = fileStream.ReadLine()!))
        {
            yield return line.Split(" ").Select(long.Parse).ToArray() switch
            {
                [var ss, var ds, var l] => new RangeMap(ds, ss, l),
                var x => throw new Exception(x.ToString()),
            };
        }
    }
}

RangeSet ApplyMaps(RangeSet rangeSet, RangeMap[] maps)
{
    return rangeSet.Map(range => MapRange(range));

    IEnumerable<Range> MapRange(Range range)
    {
        var rangeStart = range.Start;
        var rangeEnd = range.Start + range.Length;

        // Look for the first mapping that start after the range start
        var (index, value) = maps.Enumerate().FirstOrDefault(x => x.value.sourceStart >= rangeStart);
        
        ...
    }
}

record Range(long Start, long Length)
{
    public Range MergeWith(Range range)
    {
        // Ensure range start after this
        if (range.Start < Start) return range.MergeWith(this);

        // Ensure the ranges overlap
        if (Start + Length < range.Start) throw new Exception($"Cannot merge: {Start + Length} < {range.Start}");

        return new(Start, Math.Max(Length, range.Start + range.Length - Start));
    }

    public override string ToString() => $"{Start}:{Length}";
}



class RangeSet
{
    LinkedList<Range> _rangesList = new();

    public long Min()
    {
        return _rangesList.First().Start;
    }

    public RangeSet(IEnumerable<Range>? items = null)
    {
        if (items != null)
        {
            foreach (var item in items)
            {
                AddRange(item);
            }
        }
    }
    public void AddRange(Range range)
    {
        var n =_rangesList.FindFirst(x => x.Start > range.Start);
        if (n == null)
        {
            // All existing ranges start before the new one
            // Maybe the last one include the new one
            n = _rangesList.Last;
            if (n == null)
            {
                _rangesList.AddLast(range);
                return;
            }

            var lastRange = n.Value;
            var lastRangeLastIndex = lastRange.Start + lastRange.Length - 1;
            if (lastRangeLastIndex < range.Start - 1)
            {
                // nope, the new range is after
                _rangesList.AddLast(range);
                return;
            }
            else
            {
                // yep, we can merge them
                n.Value = lastRange.MergeWith(range);
            }
        }
        else
        {
            // The node start is after range
            // Can range be merged with the next node?
            var canMergeWithNext = range.Start + range.Length >= n.Value.Start;

            // Can it be merged with the previous ?
            var previous = n.Previous?.Value;
            var canMergeWithPrevious = previous != null && previous.Start + previous.Length >= range.Start;

            if (canMergeWithNext && canMergeWithPrevious)
            {
                n.Value = n.Value.MergeWith(range).MergeWith(previous!);
                _rangesList.Remove(n.Previous!);
            }
            else if (canMergeWithPrevious) n.Previous!.Value = n.Previous.Value.MergeWith(range);
            else if (canMergeWithNext) n.Value = range.MergeWith(n.Value);
            else
            {
                _rangesList.AddBefore(n, range);
            }
        }
    }

    public RangeSet Map(Func<Range, IEnumerable<Range>> func)
    {
        var result = new RangeSet();
        foreach (var item in _rangesList.SelectMany(r => func(r)))
        {
            result.AddRange(item);
        }

        return result;
    }

    public override string ToString() => $"[{string.Join(",", _rangesList)}]";

    internal RangeSet Merge(RangeSet other)
    {
        var result = new RangeSet();
        foreach (var item in _rangesList) result.AddRange(item);
        foreach (var item in other._rangesList) result.AddRange(item);
        return result;
    }
}

record RangeMap(long sourceStart, long destinationStart, long length)
{
    public override string ToString() => $"{sourceStart}->{destinationStart} ({length})";

    public IEnumerable<Range> Map(Range range)
    {
        if (range.Start + range.Length <= sourceStart) return [range];
        if (range.Start >= sourceStart + length) return [range];

        // ------s------------e------   range
        // --ms------me-------------    map 1
        // ---------ms---me---------    map 2
        // -------------ms------me--    map 3
        // ----------------------ms--

        var ms = Math.Max(sourceStart, range.Start);
        var me = Math.Min(sourceStart + length, range.Start + range.Length);
        IEnumerable<Range> GetMap()
        {
            // Yield part before the mapped region: unchanged
            if (range.Start < ms) yield return new Range(range.Start, ms - range.Start);

            // Yield the mapped part (from ms to me)
            yield return new Range(destinationStart + ms - sourceStart, me - ms);

            // Yield the part after the mapped region, unchanged
            if (range.Start + range.Length > me) yield return new Range(me, range.Start + range.Length - me);
        }

        return GetMap();
    }
}