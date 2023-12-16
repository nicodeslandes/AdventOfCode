using MapType = int;
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
            .Aggregate(seeds, ApplyMaps);

    return rangeSet.Min();
}

(RangeSet seeds, Dictionary<MapType, RangeMap[]> maps) ReadInput()
{
    using var fileStream = File.OpenText(args[0]) ?? throw new Exception("File open");
    var seeds = fileStream.ReadLine()?.Split(": ") switch
    {
        ["seeds", var seedValues] => new RangeSet(
            seedValues.Split(" ").Select(long.Parse).Buffer(2).Select(pair => new Range(pair[0], pair[0] + pair[1]))),
        var x => throw new Exception($"Invalid line: {x}"),
    };

    fileStream.ReadLine();

    var maps = Enumerable.Range(0, 7)
        .Select(i => KeyValuePair.Create(i, ReadRangeMaps()))
        .ToDictionary();

    return (seeds, maps);

    RangeMap[] ReadRangeMaps()
    {
        return ReadRangeMapsRaw().OrderBy(m => m.from.Start).ToArray();
    }

    IEnumerable<RangeMap> ReadRangeMapsRaw()
    {
        fileStream.ReadLine();
        string line;
        while (!string.IsNullOrEmpty(line = fileStream.ReadLine()!))
        {
            yield return line.Split(" ").Select(long.Parse).ToArray() switch
            {
                [var ds, var ss, var l] => new RangeMap(new Range(ss, ss + l), ds - ss),
                var x => throw new Exception(x.ToString()),
            };
        }
    }
}

RangeSet ApplyMaps(RangeSet rangeSet, RangeMap[] maps)
{
    Console.WriteLine("From: {0}; maps: {1}", rangeSet, maps.StringJoin());
    var result = rangeSet.Map(range => MapRange(range));
    Console.WriteLine("To:   {0}", result);
    return result;


    IEnumerable<Range> MapRange(Range range)
    {
        // Range: -------------s------------------e----------------
        // Maps : -m1--e1--m2-----e2--m3--e3--m4----e4---m5---e5---

        var rangeEnd = range.End;
        var sourcePosition = range.Start;

        foreach(var mapRange in maps)
        {
            // Skip mapRanges that end before s
            if (mapRange.from.End <= sourcePosition) continue;

            // If the map range start after e, we're done
            if (mapRange.from.Start >= rangeEnd) break;

            //Console.WriteLine("Range {0}; processing mapping {1}", range, mapRange);
            // Bring the map start/end within the current range
            var mapStart = Math.Max(mapRange.from.Start, sourcePosition);
            var mapEnd = Math.Min(mapRange.from.End, rangeEnd);

            //Console.WriteLine("Adjusted mapping range: [{0}, {1})", mapStart, mapEnd);

            // from sourcePos to mapStart, no mapping: we issue the range unshifted
            if (sourcePosition < mapStart) yield return new Range(sourcePosition, mapStart);

            // from mapStart to mapEnd, we shift by the mapping offset
            if (mapEnd > mapStart) yield return new Range(mapStart + mapRange.offset, mapEnd + mapRange.offset);

            // we stop here; the next mapping might map the rest of the range
            sourcePosition = mapEnd;

            // Any range left to map?
            if (sourcePosition >= rangeEnd) break;
        }

        // Issue any leftover range with an unshifted range
        if (sourcePosition < rangeEnd) yield return new Range(sourcePosition, rangeEnd);
    }
}

record Range(long Start, long End)
{
    public Range MergeWith(Range range)
    {
        // Ensure range start after this
        if (range.Start < Start) return range.MergeWith(this);

        // Ensure the ranges overlap
        if (End < range.Start) throw new Exception($"Cannot merge: {End} < {range.Start}");

        return new(Start, Math.Max(End, range.End));
    }

    public override string ToString() => $"[{Start}-{End})";
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
            if (lastRange.End < range.Start)
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
            var canMergeWithNext = range.End >= n.Value.Start;

            // Can it be merged with the previous ?
            var previous = n.Previous?.Value;
            var canMergeWithPrevious = previous != null && previous.End >= range.Start;

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

    public override string ToString() => $"[{_rangesList.StringJoin()}]";

    internal RangeSet Merge(RangeSet other)
    {
        var result = new RangeSet();
        foreach (var item in _rangesList) result.AddRange(item);
        foreach (var item in other._rangesList) result.AddRange(item);
        return result;
    }
}

record RangeMap(Range from, long offset)
{
    public override string ToString() => $"{from} ({offset:+0;-#})";

    public IEnumerable<Range> Map(Range range)
    {
        if (range.End <= from.Start) return [range];
        if (range.Start >= from.End) return [range];

        // ------s------------e------   range
        // --ms------me-------------    map 1
        // ---------ms---me---------    map 2
        // -------------ms------me--    map 3
        // ----------------------ms--

        var ms = Math.Max(from.Start, range.Start);
        var me = Math.Min(from.End, range.End);
        IEnumerable<Range> GetMap()
        {
            // Yield part before the mapped region: unchanged
            if (range.Start < ms) yield return new Range(range.Start, ms);

            // Yield the mapped part (from ms to me)
            yield return new Range(ms + offset, me + offset);

            // Yield the part after the mapped region, unchanged
            if (range.End > me) yield return new Range(me, range.End);
        }

        return GetMap();
    }
}