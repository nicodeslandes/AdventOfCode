Console.WriteLine("Part1: {0}", Part1());
Console.WriteLine("Part2: {0}", Part2());

long Part1()
{
    var (seeds, maps) = ReadInput();

    return seeds.Min(i =>
    {
        var seed = i;
        i = Map(i, maps.SeedToSoil);
        i = Map(i, maps.SoilToFertilizer);
        i = Map(i, maps.FertilizerToWater);
        i = Map(i, maps.WaterToLight);
        i = Map(i, maps.LightToTemperature);
        i = Map(i, maps.TemperatureToHumidity);
        i = Map(i, maps.HumidityToLocation);

        //Console.WriteLine($"Seed {seed} -> {i}");
        return i;
    });

    long Map(long value, RangeMap[] rangeMaps)
    {
        var mapped = rangeMaps.SelectNonNull(m => m.Map(value)).FirstOrDefault();
        return mapped == 0 ? value : mapped;
    }
}

long Part2()
{
    var input = ReadInput();
    return 0;
}

(long[] seeds, Maps maps) ReadInput()
{
    using var fileStream = File.OpenText(args[0]) ?? throw new Exception("File open");
    var seeds = fileStream.ReadLine()?.Split(": ") switch
    {
        ["seeds", var seedValues] => seedValues.Split(" ").Select(long.Parse).ToArray(),
        var x => throw new Exception($"Invalid line: {x}"),
    };

    fileStream.ReadLine();

    var maps = new Maps(
        ReadRangeMaps(),
        ReadRangeMaps(),
        ReadRangeMaps(),
        ReadRangeMaps(),
        ReadRangeMaps(),
        ReadRangeMaps(),
        ReadRangeMaps()
        );

    return (seeds, maps);

    RangeMap[] ReadRangeMaps() =>
        ReadRangeMapsRaw().OrderBy(m => m.sourceStart).ToArray();

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

record RangeMap(long sourceStart, long destinationStart, long length)
{
    public long? Map(long src) => src < sourceStart ? src :
        src >= sourceStart && src < sourceStart + length ? destinationStart + src - sourceStart :
        null;
}

record Maps(
    RangeMap[] SeedToSoil,
    RangeMap[] SoilToFertilizer,
    RangeMap[] FertilizerToWater,
    RangeMap[] WaterToLight,
    RangeMap[] LightToTemperature,
    RangeMap[] TemperatureToHumidity,
    RangeMap[] HumidityToLocation);