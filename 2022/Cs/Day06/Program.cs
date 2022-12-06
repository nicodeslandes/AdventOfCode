Console.WriteLine("Part1: {0}", Part1());
Console.WriteLine("Part2: {0}", Part2());


int Part1()
{
    var signal = ReadInput();
    return signal
        .Select((ch, index) => (ch, index))
        .OverlappingBuffer(4)
        .Where(buffer => buffer.Select(t => t.ch).ToHashSet().Count == 4)
        .Select(buffer => buffer[^1].index + 1)
        .First();
}

int Part2()
{
    var signal = ReadInput();
    return signal
        .Select((ch, index) => (ch, index))
        .OverlappingBuffer(14)
        .Where(buffer => buffer.Select(t => t.ch).ToHashSet().Count == 14)
        .Select(buffer => buffer[^1].index + 1)
        .First();
}

string ReadInput()
{
    return File.ReadAllText(args[0]);
}

public static class Ext
{
    public static IEnumerable<T[]> OverlappingBuffer<T>(this IEnumerable<T> src, int count)
    {
        var queue = new Queue<T>();
        foreach (var item in src)
        {
            queue.Enqueue(item);
            if (queue.Count == count)
            {
                yield return queue.ToArray();
                queue.Dequeue();
            }
        }
    }
}