Console.WriteLine("Part1: {0}", Part1());
Console.WriteLine("Part2: {0}", Part2());

int Part1()
{
    var input = ReadInput();

    return input.Sum(card => card.cards.Where(card.winning.Contains).Aggregate(0, (score, _) => score == 0 ? 1 : score * 2));
}

int Part2()
{
    return 0;
}

IEnumerable<(HashSet<int> winning, int[] cards)> ReadInput()
{
    return File.ReadLines(args[0])
        .Select(l => l.Split(": ")[1].Split(" | ").Select(n => n.Split(" ", StringSplitOptions.RemoveEmptyEntries).Select(int.Parse)).ToArray() switch
        {
            [var w, var c] => (winning: w.ToHashSet(), cards: c.ToArray()),
            var x => throw new NotSupportedException(x.ToString()),
        });
}