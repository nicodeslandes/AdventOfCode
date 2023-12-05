Console.WriteLine("Part1: {0}", Part1());
Console.WriteLine("Part2: {0}", Part2());

int Part1()
{
    var input = ReadInput();

    return input.Sum(card => card.matchCount == 0 ? 0 : 1 << (card.matchCount - 1));
}

int Part2()
{
    var input = ReadInput();
    var counts = input
        .Aggregate(ImmutableDictionary.Create<int, int>(),
        (counts, c) =>
        {
            var currentCardCount = counts.GetValueOrDefault(c.nb) + 1;
            counts = counts.SetItem(c.nb, currentCardCount);
            counts = counts.SetItems(Enumerable.Range(c.nb + 1, c.matchCount).Select(newCard => KeyValuePair.Create(newCard, counts.GetValueOrDefault(newCard) + currentCardCount)));
            return counts;
        });
    return counts.Values.Sum();
}

IEnumerable<(int nb, HashSet<int> winning, int matchCount)> ReadInput()
{
    return File.ReadLines(args[0])
        .Enumerate()
        .Select(l => (l.index, value: l.value.Split(": ")[1].Split(" | ").Select(n => n.Split(" ", StringSplitOptions.RemoveEmptyEntries).Select(int.Parse)).ToArray() switch
        {
            [var w, var c] => (winning: w.ToHashSet(), cards: c),
            var x => throw new NotSupportedException(x.ToString()),
        }))
        .Select(t => (nb: t.index + 1, t.value.winning, matchCount: t.value.cards.Count(t.value.winning.Contains)));
}