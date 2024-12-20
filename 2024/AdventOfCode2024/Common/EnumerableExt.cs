using System.Diagnostics.CodeAnalysis;
using System.Numerics;
using System.Runtime.CompilerServices;

namespace Common;

public static class EnumerableExt
{
    public static T Multiply<T>(this IEnumerable<T> src) where T : INumber<T>
        => src.Aggregate(T.MultiplicativeIdentity, (current, x) => current * x);

    public static IEnumerable<T> Diff<T>(this IEnumerable<T> src) where T : INumber<T>
    {
        T? prev = default;
        bool first = true;
        foreach (var item in src)
        {
            if (first)
            {
                first = false;
            }
            else
            {
                yield return item - prev!;
            }

            prev = item;
        }
    }

    public static IEnumerable<(T current, T next)> GroupWithNext<T>(this IEnumerable<T> src)
    {
        T current = default!;
        bool first = true;
        foreach (var x in src)
        {
            if (first)
            {
                first = false;
            }
            else
            {
                yield return (current, x);
            }

            current = x;
        }
    }

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

    public static IEnumerable<(T x, T y)> GetAllPairs<T>(this T[] values)
    {
        for (int i = 0; i < values.Length; i++)
        {
            for (int j = i + 1; j < values.Length; j++)
            {
                yield return (values[i], values[j]);
            }
        }
    }
}