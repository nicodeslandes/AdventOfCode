namespace Common;

public class LongEnumerable
{
    public static IEnumerable<long> Range(long start, long count)
    {
        for (var i = start; i < start + count; i++)
            yield return i;
    }
}
