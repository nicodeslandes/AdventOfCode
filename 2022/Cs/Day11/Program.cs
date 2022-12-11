using System.Linq.Expressions;
using System.Reflection;
using System.Security.Cryptography.X509Certificates;

Console.WriteLine("Part1: {0}", Part1());
Console.WriteLine("Part2: {0}", Part2());


int Part1()
{
    var monkeys = ReadInput().ToArray();
    var monkeyBusiness = new int[monkeys.Length];

    for (var round = 0; round < 20; round++)
    {
        foreach (var monkey in monkeys)
        {
            while (monkey.Items.Count > 0)
            {
                monkeyBusiness[monkey.Id]++;
                var itemWorryLevel = monkey.Items.Dequeue();
                itemWorryLevel = monkey.Operation(itemWorryLevel);
                itemWorryLevel /= 3;
                var destination = monkey.ThrowAction(itemWorryLevel);
                monkeys[destination].Items.Enqueue(itemWorryLevel);
            }
        }
    }
    return monkeyBusiness.OrderDescending().Take(2).Multiply();
}

int Part2()
{
    return 0;
}

IEnumerable<Monkey> ReadInput()
{
    var linesEnumerator = File.ReadLines(args[0]).GetEnumerator();
    while (linesEnumerator.MoveNext())
    {
        string NextLine() { linesEnumerator!.MoveNext(); return  linesEnumerator.Current; }
        var line = linesEnumerator.Current;
        if (line.IsEmpty()) continue;

        var monkeyId = int.Parse(line.Split(' ')[1][..^1]);
        var items = NextLine().Split(": ")[1].Split(", ").Select(int.Parse);
        var operation = ParseOperation(NextLine().Split(": ")[1]);
        var throwTest = NextLine().Split(": ")[1];
        var throwActionTrueDestination = int.Parse(NextLine().Split(' ')[^1]);
        var throwActionFalseDestination = int.Parse(NextLine().Split(' ')[^1]);

        yield return new Monkey(monkeyId, new(items), operation,
            ParseThrowAction(throwTest, throwActionTrueDestination, throwActionFalseDestination));

        Func<int, int> ParseOperation(string operation)
        {
            var index = operation.IndexOf('=') + 2;

            // new = old + 8, new = old * old
            var oldParam = Expression.Parameter(typeof(int));

            var operand1 = ParseOperand();
            var op = NextLexem();
            var operand2 = ParseOperand();

            var operationExpression = op == "+"
                ? Expression.Add(operand1, operand2)
                : Expression.Multiply(operand1, operand2);

            return Expression.Lambda<Func<int, int>>(operationExpression, oldParam).Compile();

            string? NextLexem()
            {
                while (index < operation.Length && char.IsWhiteSpace(operation[index])) index++;
                if (index >= operation.Length) return null;

                var start = index;
                while (index < operation.Length && !char.IsWhiteSpace(operation[index])) index++;
                return operation[start..index];
            }

            Expression ParseOperand()
            {
                return NextLexem() switch
                {
                    "old" => oldParam,
                    var x => Expression.Constant(int.Parse(x)),
                };
            }
        }

        Func<int, int> ParseThrowAction(string throwTest, int trueDest, int falseDest)
        {
            // throwTest: divisible by 17
            var denominator = int.Parse(throwTest.Split(' ')[^1]);
            return worryLevel => worryLevel % denominator == 0 ? trueDest : falseDest;
        }
    }
}

record Monkey(int Id, Queue<int> Items, Func<int, int> Operation, Func<int, int> ThrowAction);