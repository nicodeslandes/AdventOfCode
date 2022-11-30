Config.ShowIntermediate = false;

Console.WriteLine("Part1: {0}", Part1());
Console.WriteLine("Part2: {0}", Part2());

int Part1()
{

    var sum = (SnailFishNumber?)null;

    foreach (var nb in ReadNumbers())
    {    
        sum = sum is null ? nb : sum + nb;
        if (Config.ShowIntermediate) Console.WriteLine("Sum: {0}", sum);
    }

    return sum!.Magnitude();
}

int Part2()
{
    var numbers = ReadNumbers().ToArray();
    var max = int.MinValue;
    for (int i = 0; i < numbers.Length; i++)
    {
        for (int j = i + 1; j < numbers.Length; j++)
        {
            var sum = numbers[i] + numbers[j];
            max = Math.Max(max, sum.Magnitude());
        }
    }

    return max;
}

IEnumerable<SnailFishNumber> ReadNumbers()
{
    var input = File.ReadLines(args[0]);
    foreach (var line in input)
    {
        var nb = SnailFishNumber.Parse(line);
        yield return nb;
    }
}
static class Config
{
    public static bool ShowIntermediate;
}

class SnailFishNumber
{
    private LinkedList<Lexem> _lexems;
    private SnailFishNumber(IEnumerable<Lexem> lexems)
    {
        _lexems = new(lexems);
    }

    public int Magnitude()
    {
        var stack = new Stack<int>();
        foreach (var lexem in _lexems)
        { 
            switch (lexem)
            {
                case Number(var n):
                    stack.Push(n);
                    break;
                case CloseBracket:
                    var right = stack.Pop();
                    var left = stack.Pop();
                    stack.Push(3 * left + 2 * right);
                    break;
            }
        }

        return stack.Pop();
    }

    public static SnailFishNumber operator+(SnailFishNumber x, SnailFishNumber y)
    {
        var sum = new SnailFishNumber(new IEnumerable<Lexem>[] {
            new Lexem[] { OpenBracket.Instance },
            x._lexems.Select(l => l is Number(var n) ? new Number(n) : l),
            new Lexem[] { Comma.Instance },
            y._lexems.Select(l => l is Number(var n) ? new Number(n) : l),
            new Lexem[] { CloseBracket.Instance }
        }.Concat());

        while (sum.Reduce()) ;
        return sum;
    }

    public override string ToString() => string.Join("", _lexems.Select(x =>
    x switch
    {
        OpenBracket => "[",
        CloseBracket => "]",
        Comma => ",",
        Number(var n) => n.ToString(),
        _ => throw new Exception($"Unhandled value: {x}")
    }));

    public static SnailFishNumber Parse(string input)
    {
        var lexems = new List<Lexem>();
        int? currentNumber = null;

        foreach (var ch in input)
        {
            if (!char.IsDigit(ch))
                AddPendingNumber();

            switch (ch)
            {
                case '[':
                    lexems.Add(OpenBracket.Instance);
                    break;
                case ']':
                    lexems.Add(CloseBracket.Instance);
                    break;
                case ',':
                    lexems.Add(Comma.Instance);
                    break;
                case var d when char.IsDigit(d):
                    currentNumber = (currentNumber ?? 0) * 10 + (d - '0');
                    break;
            }
        }

        AddPendingNumber();

        return new(lexems);
        void AddPendingNumber()
        {
            if (currentNumber is { } number)
            {
                lexems.Add(new Number(number));
                currentNumber = null;
            }
        }
    }

    private enum ReductionStep
    {
        Explode,
        Split,
    }

    public bool Reduce()
    {
        if (Reduce(ReductionStep.Explode)) return true;
        return Reduce(ReductionStep.Split);
    }

    private bool Reduce(ReductionStep step)
    {
        // Find a 4-level pair
        var level = 0;
        var currentNode = _lexems.First;

        var lastNumberNode = (LinkedListNode<Lexem>?)null;
        var numberToAdd = (int?)null;

        LinkedListNode<Lexem>? Next() => (currentNode = currentNode!.Next);
        while (currentNode != null)
        {
            switch (currentNode.Value)
            {
                case OpenBracket:
                    if (step == ReductionStep.Explode && level == 4 && numberToAdd is null)
                    {
                        // Explode the pair
                        var parent = currentNode.Previous;
                        // 1. Get the (left,right) members of the pair
                        var left = (Number)Next()!.Value;
                        Next();
                        var right = (Number)Next()!.Value;

                        // 2. Add left to the previous number
                        if (lastNumberNode is not null)
                        {
                            ((Number)lastNumberNode.Value).Value += left.Value;
                        }

                        // 3. Add right to the next number
                        numberToAdd = right.Value;

                        // Move to the next lexem
                        Next();
                        Next();

                        //Console.WriteLine(this);

                        // Remove node
                        _lexems.Remove(parent!.Next!.Next!.Next!.Next!.Next!);
                        _lexems.Remove(parent!.Next!.Next!.Next!.Next!);
                        _lexems.Remove(parent!.Next!.Next!.Next!);
                        _lexems.Remove(parent!.Next!.Next!);
                        _lexems.Remove(parent!.Next!);

                        // And replace it with '0'
                        _lexems.AddAfter(parent!, new Number(0));
                    }
                    level++;
                    break;
                case CloseBracket:
                    level--;
                    break;
                case Number number:
                    if (numberToAdd is { } n)
                    {
                        number.Value += n;
                        if (Config.ShowIntermediate) Console.WriteLine("Explode: {0}", this);
                        return true;
                    }

                    if (step == ReductionStep.Split && number.Value >= 10)
                    {
                        var parent = currentNode.Previous;
                        _lexems.Remove(currentNode);
                        _lexems.AddAfter(parent!, CloseBracket.Instance);
                        _lexems.AddAfter(parent!, new Number((number.Value + 1)/ 2));
                        _lexems.AddAfter(parent!, Comma.Instance);
                        _lexems.AddAfter(parent!, new Number(number.Value / 2));
                        _lexems.AddAfter(parent!, OpenBracket.Instance);

                        if (Config.ShowIntermediate) Console.WriteLine("Split  : {0}", this);
                        return true;
                    }
                    lastNumberNode = currentNode;
                    break;
            }
            Next();
        }

        if (numberToAdd is not null)
        {
            if (Config.ShowIntermediate) Console.WriteLine("Explode: {0}", this);
            return true;
        }
        return false;
    }
}

abstract record Lexem;
record OpenBracket : Lexem
{
    private OpenBracket() {}

    public static readonly OpenBracket Instance = new();
}
record CloseBracket : Lexem
{
    private CloseBracket() { }

    public static readonly CloseBracket Instance = new();
}

record Comma : Lexem
{
    private Comma() { }

    public static readonly Comma Instance = new();
}

record Number : Lexem
{
    public int Value { get; set; }

    public Number(int value)
    {
        Value = value;
    }

    public void Deconstruct(out int n)
    {
        n = Value;
    }
}