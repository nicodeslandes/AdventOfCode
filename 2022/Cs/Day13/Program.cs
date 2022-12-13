Console.WriteLine("Part1: {0}", Part1());
Console.WriteLine("Part2: {0}", Part2());


int Part1()
{
    return ReadInput()
        .Enumerate()
        .Where(x => x.value.left.CompareTo(x.value.right) <= 0)
        .Sum(x => x.index + 1);
}

int Part2()
{
    return 0;
}

IEnumerable<(Element left, Element right)> ReadInput()
{
    Element previous = null;
    foreach (var line in Utils.ReadLinesFromInputFile(args))
    {
        if (string.IsNullOrEmpty(line)) continue;
        if (previous == null)
        {
            previous = ParseElement(line, out _);
        }
        else
        {
            yield return (previous, ParseElement(line, out _));
            previous = null;
        }
    }

    Element ParseElement(ReadOnlySpan<char> src, out int endIndex)
    {
        if (src[0] == '[')
        {
            // Parse list
            var items = new List<Element>();
            var i = 1;
            while (src[i] != ']')
            {
                items.Add(ParseElement(src[i..], out var end));
                i = src[end + i] == ',' ? end + i + 1 : end + i;
            }

            endIndex = i + 1;
            return new ItemList(items);
        }

        // Single number
        var j = 0;
        var number = 0;
        while (char.IsDigit(src[j]))
        {
            number = number * 10 + (src[j++] - '0');
        }

        endIndex = j;
        return new Number(number);
    }
}

abstract record Element : IComparable<Element>
{
    public abstract int CompareTo(Element? other);
}

record Number(int Value): Element
{
    public override int CompareTo(Element? other)
    {
        if (other == null) return 1;
        switch (other)
        {
            case Number(var v):
                return Value.CompareTo(v);
            case ItemList x:
                return (new ItemList(new() { this })).CompareTo(x);
            default:
                throw new Exception("What!??");
        }
    }
}

record ItemList(List<Element> Items) : Element
{
    public override int CompareTo(Element? other)
    {
        if (other == null) return 1;
        switch (other)
        {
            case Number n:
                return CompareTo(new ItemList(new List<Element> { n }));
            case ItemList(var otherItems):
                int i = 0;
                for (; i < Items.Count; i++)
                {
                    if (otherItems.Count <= i) return 1;
                    var compare = Items[i].CompareTo(otherItems[i]);
                    if (compare != 0) return compare;
                }

                return otherItems.Count > i ? -1 : 0;
            default:
                throw new Exception("What!??");
        }
    }
}