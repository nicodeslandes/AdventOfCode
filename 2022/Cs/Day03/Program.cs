﻿using System.Collections.Immutable;

Console.WriteLine("Part1: {0}", Part1());
Console.WriteLine("Part2: {0}", Part2());

// 1 - Rock
// 2 - Paper
// 3 - Scissor

int Part1()
{
    return ReadInput()
        .Select(line =>
        {
            var chars = new HashSet<char>();
            char? dup = null;
            foreach (var ch in line.Take(line.Length / 2)) chars.Add(ch);

            foreach (var ch in line.Skip(line.Length / 2))
            {
                if (chars.Contains(ch))
                {
                    dup = ch;
                    break;
                }
            }
            return GetPriority(dup!.Value);
        })
        .Sum();
}

int GetPriority(char ch) =>
    char.IsLower(ch) ? ch - 'a' + 1 : ch - 'A' + 27;

int Part2()
{
    return ReadInput()
        .Buffer(3)
        .Select(group => group
            .Select(x => ImmutableHashSet.Create(x.ToArray()))
            .Aggregate((ch1, ch2) => ch1.Intersect(ch2))
            .Single())
        .Sum(GetPriority);
}

IEnumerable<string> ReadInput()
{
    return File.ReadLines(args[0]);
}
