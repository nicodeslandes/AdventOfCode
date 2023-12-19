using System.Runtime.InteropServices;

Console.WriteLine("Part1: {0}", Part1());
Console.WriteLine("Part2: {0}", Part2());

long Part1()
{
    var (workflows, parts) = ReadInput();

    return parts.Where(p => ExecuteWorkflow(p, "in", workflows) == "A").Sum(p => p.Attributes.Values.Sum());
}

long Part2()
{
    var (workflows, parts) = ReadInput();
    return 0;
}

string ExecuteWorkflow(Part part, string current, IDictionary<string, Workflow> workflows)
{
    while (current is not ("A" or "R"))
    {
        foreach (var rule in workflows[current].Rules)
        {
            if (IsRuleMatch(part, rule))
            {
                current = rule.Destination;
                break;
            }
        }
    }

    return current;
}

bool IsRuleMatch(Part part, Rule rule)
{
    if (rule.Condition is not { } condition)
    {
        return true;
    }

    var partValue = part.Attributes[condition.Attribute];
    switch (condition.Operator)
    {
        case '>': return partValue > condition.Value;
        case '<': return partValue < condition.Value;
        default: throw new Exception($"Unhandled operator: {condition.Operator}");
    }
}

(ImmutableDictionary<string, Workflow> workflows, ImmutableArray<Part> pieces) ReadInput()
{
    var lines = Utils.ReadLinesFromInputFile(args);
    var iterator = lines.GetEnumerator();
    IEnumerable<Workflow> ReadWorkflows()
    {
        while (iterator!.MoveNext())
        {
            var line = iterator.Current;
            if (line == "") break;
            if (line.Split("{") is [var name, var ruleString])
            {
                var rules = ruleString[..^1].Split(",")
                    .Select(r => r.Split(":") switch
                    {
                    [var conditionStr, var destination] => new Rule(new Condition(conditionStr[0], conditionStr[1], int.Parse(conditionStr[2..])), destination),
                    [var destination] => new Rule(null, destination),
                        _ => throw new Exception($"Invalid split for rule {r}"),
                    })
                    .ToImmutableArray();

                yield return new Workflow(name, rules);
            }
        }
    }

    IEnumerable<Part> ReadParts()
    {
        while (iterator!.MoveNext())
        {
            var line = iterator.Current;
            var attributes = line[1..^1].Split(",").Select(attributeString =>
                (attribute: attributeString[0], value: int.Parse(attributeString[2..])));
            yield return new Part(attributes.ToImmutableDictionary(a => a.attribute, a => a.value));
        }
    }

    return (ReadWorkflows().ToImmutableDictionary(w => w.Name), ReadParts().ToImmutableArray());
}

record Workflow(string Name, ImmutableArray<Rule> Rules);
record Part(ImmutableDictionary<char, int> Attributes);

record Rule(Condition? Condition, string Destination);
record Condition(char Attribute, char Operator, int Value);