





public static class EnumerableExt
{
    public static IEnumerable<(int index, T value)> Enumerate<T>(this IEnumerable<T> coll)
        => coll.Select((val, i) => (i, val));
}