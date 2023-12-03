using System.Diagnostics.CodeAnalysis;
using System.Numerics;

namespace Common;

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

    public static IEnumerable<U> SelectNonNull<T, U>(this IEnumerable<T> src, Func<T, U?> selector)
    {
        return src.Select(selector).Where(u => u != null)!;
    }

    public static IEnumerable<U> SelectNonNull<T, U>(this IEnumerable<T> src, Func<T, U?> selector) where U: struct
    {
        return src.Select(selector).Where(u => u != null).Select(u => u!.Value);
    }

    public static IEnumerable<T> WhereNotNull<T>(this IEnumerable<T?> src) where T: notnull
    {
        return src.Where(x => x != null)!;
    }
    public static IEnumerable<T> WhereNotNull<T>(this IEnumerable<T?> src) where T : struct
    {
        return src.Where(x => x != null).Select(x => x!.Value);
    }
}