using System.Text.RegularExpressions;

Console.WriteLine("Part1: {0}", Part1());
Console.WriteLine("Part2: {0}", Part2());


string Part1()
{
    var (stacks, moves) = ReadInput();
    foreach (var m in moves)
    {
        for (int i = 0; i < m.Count; i++)
        {
            var ch = stacks[m.From - 1].Pop();
            stacks[m.To - 1].Push(ch);
        }
    }

    return string.Join("", stacks.Select(s => s.TryPeek(out var c) ? c.ToString() : ""));
}

string Part2()
{
    var (stacks, moves) = ReadInput();
    foreach (var m in moves)
    {
        var stack = new Stack<char>();
        for (int i = 0; i < m.Count; i++)
        {
            stack.Push(stacks[m.From - 1].Pop());
        }

        while (!stack.IsEmpty()) stacks[m.To - 1].Push(stack.Pop());
    }

    return string.Join("", stacks.Select(s => s.TryPeek(out var c) ? c.ToString() : ""));
}

(Stack<char>[] stacks, List<Move> moves) ReadInput()
{
    var blocks = new List<char[]>();
    var moves = new List<Move>();
    var blocksRead = false;

    foreach (var line in File.ReadLines(args[0]))
    {
        if (!blocksRead)
        {
            if (string.IsNullOrWhiteSpace(line)) blocksRead = true;
            if (!line.Contains('[')) continue;

            var count = (line.Length + 1) / 4;
            var lineBlocks = new char[count];
            for (int i = 0; i < count; i++)
            {
                lineBlocks[i] = line[i * 4 + 1];
                if (char.IsWhiteSpace(lineBlocks[i]))
                    lineBlocks[i] = '\0';
            }
            blocks.Add(lineBlocks);
        }
        else
        {
            var m = Regex.Match(line, @"move (\d+) from (\d+) to (\d+)");
            if (m.Success)
            {
                moves.Add(new(
                    int.Parse(m.Groups[1].Value),
                    int.Parse(m.Groups[2].Value),
                    int.Parse(m.Groups[3].Value)));
            }
        }
    }

    var stacks = new Stack<char>[blocks[0].Length];
    for (int i = 0; i < stacks.Length; i++)
    {
        stacks[i] = new();
    }

    foreach (var line in blocks)
    {
        for (int i = 0; i < stacks.Length; i++)
        {
            if (line[i] != '\0') stacks[i].Push(line[i]);
        }
    }

    for (int i = 0; i < stacks.Length; i++)
    {
        stacks[i] = new(stacks[i]);
    }
    return (stacks, moves);

}

record Move(int Count, int From, int To);