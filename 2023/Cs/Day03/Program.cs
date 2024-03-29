﻿using Common;
using System.Collections.Immutable;

Console.WriteLine("Part1: {0}", Part1());
Console.WriteLine("Part2: {0}", Part2());

int Part1()
{
    var schematics = ReadSchematics();
    return schematics.PartNumbers.Where(pn => pn.AdjacentSymbols.Any()).Sum(pn => pn.Number);
}

int Part2()
{
    var schematics = ReadSchematics();
    var gearParts = schematics.PartNumbers
        .SelectMany(pn => pn.AdjacentSymbols.Where(part => part.Char == '*').Select(p => (pn.Number, Symbol: p)))
        .GroupBy(t => t.Symbol)
        .Where(g => g.Count() == 2);

    return gearParts.Sum(gearGroup => gearGroup.Select(g => g.Number).Multiply());
}

Dictionary<Position, char> ReadInput()
{

    return File.ReadLines(args[0])
        .Enumerate()
        .SelectMany(l => l.value.Enumerate().Select(ch => (x: ch.index, y: l.index, ch: ch.value)))
        .Where(x => x.ch != '.')
        .ToDictionary(x => new Position(x.x, x.y), x => x.ch);
}

Schematics ReadSchematics()
{
    var input = ReadInput();
    var maxX = input.Keys.Max(k => k.X);
    var maxY = input.Keys.Max(k => k.Y);

    var partNumbers = new List<PartNumber>();
    for (int y = 0; y <= maxY; y++)
    {
        var nb = 0;
        var adjacentSymbols = new HashSet<Symbol>();
        for (int x = 0; x <= maxX; x++)
        {
            char? ch = input.TryGetValue(new Position(x, y), out var c) ? c : null;
            switch (ch)
            {
                case { } d when char.IsDigit(d):
                    nb = nb * 10 + (d - '0');
                    adjacentSymbols.AddRange(GetAdjacentSymbols(new(x, y)).Where(s => !char.IsDigit(s.Char)));
                    break;
                default:
                    AddPendingPartNumber();
                    break;

            }
        }

        AddPendingPartNumber();

        void AddPendingPartNumber()
        {
            if (nb == 0) return;
            partNumbers.Add(new(nb, adjacentSymbols.ToImmutableHashSet()));
            nb = 0;
            adjacentSymbols.Clear();
        }
    }

    IEnumerable<Symbol> GetAdjacentSymbols(Position pos) =>
        pos
            .AdjacentPositions()
            .SelectNonNull(p => input.TryGetValue(p, out var ch) ? (Symbol?)new Symbol(p, ch) : null);

    return new Schematics(maxX, maxY, partNumbers.ToImmutableArray());
}

record Schematics(int X, int Y, ImmutableArray<PartNumber> PartNumbers);

record PartNumber(int Number, ImmutableHashSet<Symbol> AdjacentSymbols)
{
    public override string ToString()
        => $"{Number} ({AdjacentSymbols.StringJoin()})";
}

record struct Symbol(Position Position, char Char);