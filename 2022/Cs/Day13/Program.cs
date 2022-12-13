RunAndMeasureTime("Part1", Part1);
RunAndMeasureTime("Part2", Part2);

int Part1()
{
    return ReadInput()
        .Buffer(2)
        .Enumerate()
        .Where(x => x.value[0] <= x.value[1])
        .Sum(x => x.index + 1);
}

int Part2()
{
    var elements = ReadInput().ToList();
    var two = Element.Parse("[[2]]");
    var six = Element.Parse("[[6]]");
    elements.Add(two);
    elements.Add(six);

    elements.Sort();

    return (elements.IndexOf(two) + 1) * (elements.IndexOf(six) + 1);
}

IEnumerable<Element> ReadInput()
{
    foreach (var line in Utils.ReadLinesFromInputFile(args))
    {
        if (string.IsNullOrEmpty(line)) continue;
        yield return Element.Parse(line);
    }
}

abstract record Element : IComparable<Element>
{
    public static Element Parse(string src)
    {
        return Parse(src, out _);

        static Element Parse(ReadOnlySpan<char> src, out int endIndex)
        {
            if (src[0] == '[')
            {
                // Parse list
                var items = new List<Element>();
                var i = 1;
                while (src[i] != ']')
                {
                    items.Add(Parse(src[i..], out var end));
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

    public abstract int CompareTo(Element? other);

    public static bool operator <=(Element x, Element y) => x.CompareTo(y) <= 0;
    public static bool operator >=(Element x, Element y) => x.CompareTo(y) >= 0;
}

record Number(int Value): Element
{
    public override int CompareTo(Element? other) => other switch
    {
        null => 1,
        Number(var v) => Value.CompareTo(v),
        ItemList x => new ItemList(this).CompareTo(x),
        _ => throw new Exception("What!??"),
    };
}

record ItemList(List<Element> Items) : Element
{
    public ItemList(Element singleItem) : this(new List<Element> { singleItem }) { }

    public override int CompareTo(Element? other)
    {
        return other switch
        {
            null => 1,
            Number n => CompareTo(new ItemList(n)),
            ItemList(var otherItems) => CompareItemList(otherItems),
            _ => throw new Exception("What!??"),
        };

        int CompareItemList(List<Element> otherItems)
        {
            int i = 0;
            for (; i < Items.Count; i++)
            {
                if (otherItems.Count <= i) return 1;
                var compare = Items[i].CompareTo(otherItems[i]);
                if (compare != 0) return compare;
            }

            return otherItems.Count > i ? -1 : 0;
        }
    }
}