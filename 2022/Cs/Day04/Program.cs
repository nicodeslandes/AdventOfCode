﻿using System.Collections.Immutable;

Console.WriteLine("Part1: {0}", Part1());
Console.WriteLine("Part2: {0}", Part2());

// 1 - Rock
// 2 - Paper
// 3 - Scissor

int Part1()
{
    return ReadInput()
        .Count(pair =>
        {
            var (first, second) = pair;
            if (first.Start > second.Start || first.Start == second.Start && first.End < second.End) (first, second) = (second, first);
            return first.Start <= second.Start && first.End >= second.End;
        });
}

int GetPriority(char ch) =>
    char.IsLower(ch) ? ch - 'a' + 1 : ch - 'A' + 27;

int Part2()
{
    return ReadInput()
        .Count(pair =>
        {
            var (first, second) = pair;
            return first.Start >= second.Start && first.Start <= second.End ||
                second.Start >= first.Start && second.Start <= first.End;
        });
}

IEnumerable<(Range first, Range second)> ReadInput()
{
    return File.ReadLines(args[0])
        .Select(l => l.Split(',').Select(range =>
        {
            var values = range.Split('-').Select(int.Parse).ToArray();
            return new Range(values[0], values[1]);
        }).ToArray())
        .Select(pair => (pair[0], pair[1]));
}

record Range(int Start, int End);