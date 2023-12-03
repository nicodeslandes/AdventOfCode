using System.Numerics;

public static class EnumerableExt
{
    public static IEnumerable<(int index, T value)> Enumerate<T>(this IEnumerable<T> coll)
        => coll.Select((val, i) => (i, val));

    public static T Multiply<T>(this IEnumerable<T> src) where T : INumber<T>
        => src.Aggregate(T.MultiplicativeIdentity, (current, x) => current * x);

    public static void AddRange<T>(this ISet<T> set, IEnumerable<T> items)
    {
        foreach (var item in items)
            set.Add(item);
    }

    public static string StringJoin<T>(this IEnumerable<T> src, string? separator = null)
        => string.Join(separator ?? ",", src);
}