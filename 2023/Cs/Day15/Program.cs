using System.Collections;
using System.Collections.Specialized;
using Move = (int dx, int dy);


Console.WriteLine("Part1: {0}", Part1());
Console.WriteLine("Part2: {0}", Part2());

long Part1()
{
    var input = ReadInput();


    return input
        .Select(CalculateHash)
        .Sum();
}

long Part2()
{
    var input = ReadInput();
    Dictionary<int, Box> boxes = new();

    foreach (var instruction in input)
    {
        var actionIndex = instruction.IndexOfAny(['-', '=']);
        var label = instruction[..actionIndex];

        var boxId = CalculateHash(label);
        if (!boxes.TryGetValue(boxId, out var box))
        {
            box = new(boxId);
            boxes[boxId] = box;
        }

        if (instruction[actionIndex] == '=')
        {
            var focal = int.Parse(instruction[(actionIndex + 1)..]);

            box.AddLens(label, focal);
        }
        else
        {
            box.RemoveLens(label);
        }
    }

    return boxes.Values.Sum(b => b.FocusingPower);
}

int CalculateHash(string instruction)
{
    var hash = 0;
    foreach (var ch in instruction)
    {
        hash += ch;
        hash *= 17;
        hash %= 256;
    }

    return hash;
}

IEnumerable<string> ReadInput()
{
    return Utils.ReadLinesFromInputFile(args).First().Split(",");
}

public class Box(int id)
{
    public int Id { get; } = id;

    private OrderedDictionary _lenses = new();

    public void AddLens(string label, int focal)
    {
        if (_lenses.Contains(label))
        {
            _lenses[label] = focal;
        }
        else
        {
            _lenses.Insert(_lenses.Count, label, focal);
        }
    }

    public void RemoveLens(string label)
    {
        _lenses.Remove(label);
    }

    public int FocusingPower
    {
        get
        {
            var lensPowers = _lenses
                .Cast<DictionaryEntry>()
                .Enumerate()
                .Select(x => (1 + Id) * (x.index + 1) * (int)(x.value.Value!))
                .ToArray();

            return lensPowers
                .Sum();
        }
    }
}