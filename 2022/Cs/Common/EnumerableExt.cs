using System.Numerics;

public static class EnumerableExt
{
    public static IEnumerable<(int index, T value)> Enumerate<T>(this IEnumerable<T> coll)
        => coll.Select((val, i) => (i, val));

    public static T Multiply<T>(this IEnumerable<T> src) where T : INumber<T>
        => src.Aggregate(T.MultiplicativeIdentity, (current, x) => current * x);
}