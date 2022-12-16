using System.Collections.Immutable;
using System.Text.RegularExpressions;

RunAndMeasureTime("Part1", Part1);
RunAndMeasureTime("Part2", Part2);

int Part1()
{
    var nodes = ReadInput();

    Console.WriteLine("Non-0 valves: {0}", string.Join(", ",nodes.Values.Where(v => v.FlowRate != 0).Select(v => v.Name)));

    return Solve(nodes);
}

int Part2()
{
    return 0;
}

int Solve(Dictionary<string, Node> nodes)
{
    // For each possible state at time t, what is the released pressure at that point?
    var states = new Dictionary<StateKey, State> { [new(0, "AA")] = new(0, 0) };
    for (int time = 0; time < 30; time++)
    {
        Console.Write("{0}", time % 10);
        Console.Out.Flush();
        var newStates = new Dictionary<StateKey, State>();

        foreach (var (stateKey, state) in states)
        {
            var current = nodes[stateKey.CurrentNode];

            // Option 1: Open the tap
            if (current.FlowRate != 0 && !IsValveOpen(current, stateKey.OpenNodes))
            {
                var s = stateKey with { OpenNodes = stateKey.OpenNodes | (1 << current.Index) };
                var candidate = new State(state.TotalRate + current.FlowRate, state.ReleasedPressure + state.TotalRate);

                // If the state is already there with a better released pressure, ignore the candidate
                if (!newStates.TryGetValue(s, out var existing) || existing.ReleasedPressure < candidate.ReleasedPressure)
                {
                    newStates[s] = candidate;
                }
            }

            // Option 2: Go to the next node
            foreach (var neighbour in current.Neighbours)
            {
                var s = stateKey with { CurrentNode = neighbour.Name };
                var candidate = state with { ReleasedPressure = state.ReleasedPressure + state.TotalRate };

                // If the state is already there with a better released pressure, ignore the candidate
                if (!newStates.TryGetValue(s, out var existing) || existing.ReleasedPressure < candidate.ReleasedPressure)
                {
                    newStates[s] = candidate;
                }
            }
        }

        states = newStates;
    }

    Console.WriteLine();
    return states.Values.Max(s => s.ReleasedPressure);

    bool IsValveOpen(Node node, int openState) => ((1 << node.Index) & openState) != 0;
}

Dictionary<string, Node> ReadInput()
{
    var nodes = new Dictionary<string, Node>();
    foreach (var line in ReadLinesFromInputFile(args))
    {
        var match = Regex.Match(line, @"Valve (\w+) has flow rate=(\d+); tunnels? leads? to valves? (.+)");
        if (match.Success)
        {
            var nodeName = match.Groups[1].Value;
            var rate = int.Parse(match.Groups[2].Value);
            var neighbours = match.Groups[3].Value.Split(", ");
            var neighbourNodes = new List<Node>();
            foreach (var n in neighbours)
            {
                if (!nodes.TryGetValue(n, out var nnode))
                {
                    nnode = new Node(n, 0);
                }
                neighbourNodes.Add(nnode);
            }

            if (nodes.TryGetValue(nodeName, out var node))
            {
                node.FlowRate = rate;
                node.Neighbours.AddRange(neighbourNodes);
            }
            else
            {
                nodes[nodeName] = new(nodeName, rate, neighbourNodes);
            }
        }
    }

    var index = 0;
    foreach (var node in nodes.Values)
    {
        if (node.FlowRate > 0) node.Index = index++;
    }

    return nodes;
}

record State(int TotalRate, int ReleasedPressure);
record StateKey(int OpenNodes, string CurrentNode);

record Node(string Name, List<Node> Neighbours)
{
    public Node(string name, int flowRate, List<Node>? neighbours = null)
        : this(name, neighbours ?? new())
    {
        FlowRate = flowRate;
    }

    public int FlowRate { get; set; }
    public int Index { get; set; } = -1;
}

class Grid
{
}