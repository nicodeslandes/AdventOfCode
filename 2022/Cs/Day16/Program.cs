using System.Text.RegularExpressions;

RunAndMeasureTime("Part1", Part1);
RunAndMeasureTime("Part2", Part2);

int Part1()
{
    var nodes = ReadInput();

    WriteLine("Non-0 valves: {0}", string.Join(", ",nodes.Where(v => v != null && v.FlowRate != 0).Select(v => v.Name)));

    return Solve(nodes);
}

int Part2()
{
    return 0;
}

int Solve(Node[] nodes)
{
    var closedValveCount = nodes.Count(v => v != null && v.FlowRate != 0);
    var allOpen = (1 << closedValveCount) - 1;

    // For each possible state at time t, what is the released pressure at that point?
    var states = new Dictionary<StateKey, State> { [new(0, 4)] = new(2, 2, 0, 0) };
    var allOpenResult = new Dictionary<int, int>(); // High released pressure by time

    var maxTotalFlowRate = nodes.Sum(v => v?.FlowRate ?? 0);

    for (int time = 0; time < 26; time++)
    {
        Write("{0}", time % 10);
        Out.Flush();
        var newStates = new Dictionary<StateKey, State>();

        void AddCandidate(StateKey key, State state, int addedFlowRate, int releasedPressure, int id1, int id2)
        {
            // If the state is already there with a better released pressure, ignore the candidate
            if (!newStates.TryGetValue(key, out var existing) || existing.ReleasedPressure < releasedPressure)
            {
                var candidate = state with { TotalRate = state.TotalRate + addedFlowRate, ReleasedPressure = releasedPressure, Id1 = id1, Id2 = id2 };
                newStates[key] = candidate;
            }
        }

        foreach (var (stateKey, state) in states)
        {
            var current1 = nodes[state.Id1];
            var current2 = nodes[state.Id2];
            var releasedPressure = state.ReleasedPressure + state.TotalRate;

            // Option 1: Open the tap at current 1
            if (current1.FlowRate != 0 && !IsValveOpen(current1, stateKey.OpenNodes))
            {
                var openNodes = stateKey.OpenNodes | (1 << current1.OpenIndex);
                //if (openNodes == allOpen)
                //{
                //    //WriteLine("All open in {0} min! Total rate: {1}, Released Pressure so far: {2}", time, state.TotalRate + current1.FlowRate, state.ReleasedPressure + state.TotalRate);
                //    if (!allOpenResult.TryGetValue(time, out var bestReleasedPressure) || bestReleasedPressure < releasedPressure)
                //    {
                //        allOpenResult[time] = releasedPressure;
                //    }
                //}
                //else
                {
                    // Move Current2
                    foreach (var neighbour in current2.Neighbours)
                    {
                        var s = stateKey with { OpenNodes = openNodes, Current = stateKey.Current / state.Id2 * neighbour.Id };
                        AddCandidate(s, state, current1.FlowRate, releasedPressure, state.Id1, neighbour.Id);
                    }
                }
            }

            // Open tap at current2
            if (current2.FlowRate != 0 && !IsValveOpen(current2, stateKey.OpenNodes))
            {
                var openNodes = stateKey.OpenNodes | (1 << current2.OpenIndex);
                //if (openNodes == allOpen)
                //{
                //    //WriteLine("All open in {0} min! Total rate: {1}, Released Pressure so far: {2}", time, state.TotalRate + current1.FlowRate, state.ReleasedPressure + state.TotalRate);
                //    if (!allOpenResult.TryGetValue(time, out var bestReleasedPressure) || bestReleasedPressure < releasedPressure)
                //    {
                //        allOpenResult[time] = releasedPressure;
                //    }
                //}
                //else
                {
                    // Move Current1
                    foreach (var neighbour in current1.Neighbours)
                    {
                        var s = stateKey with { OpenNodes = openNodes, Current = stateKey.Current / state.Id1 * neighbour.Id };
                        AddCandidate(s, state, current2.FlowRate, releasedPressure, neighbour.Id, state.Id2);
                    }
                }
            }

            // Open both taps
            if (current1.Id != current2.Id && current1.FlowRate != 0 && !IsValveOpen(current1, stateKey.OpenNodes) && current2.FlowRate != 0 && !IsValveOpen(current2, stateKey.OpenNodes))
            {
                var openNodes = stateKey.OpenNodes | (1 << current1.OpenIndex) | (1 << current2.OpenIndex);
                //if (openNodes == allOpen)
                //{
                //    //WriteLine("All open in {0} min! Total rate: {1}, Released Pressure so far: {2}", time, state.TotalRate + current1.FlowRate, state.ReleasedPressure + state.TotalRate);
                //    if (!allOpenResult.TryGetValue(time, out var bestReleasedPressure) || bestReleasedPressure < releasedPressure)
                //    {
                //        allOpenResult[time] = releasedPressure;
                //    }
                //}
                //else
                {
                    var s = stateKey with { OpenNodes = openNodes };
                    AddCandidate(s, state, current1.FlowRate + current2.FlowRate, releasedPressure, state.Id1, state.Id2);
                }
            }

            // Option 2: Go to the next node for both
            foreach (var n1 in current1.Neighbours)
            {
                foreach (var n2 in current2.Neighbours)
                {
                    var s = stateKey with { Current = n1.Id * n2.Id };
                    AddCandidate(s, state, 0, releasedPressure, n1.Id, n2.Id);
                }
            }
        }

        // Clear out all states that cannot possibly beat the best one
        // At worst, all state will be able to produce the current flow rate until the end
        int MinExpectedPressure(State state) => state.ReleasedPressure + (25 - time) * state.TotalRate;
        int MaxExpectedPressure(State state) => state.ReleasedPressure + (25 - time) * maxTotalFlowRate;
        var bestSoFar = newStates.Values.Max(MinExpectedPressure);
        var toDelete = newStates
            .Where(kvp => MaxExpectedPressure(kvp.Value) < bestSoFar)
            .Select(kvp => kvp.Key)
            .ToList();
        toDelete.ForEach(k => newStates.Remove(k));
        states = newStates;
     }

    WriteLine();
    return /*allOpenResult.Count > 0
        ? allOpenResult.Max(kvp => kvp.Value + maxTotalFlowRate * (25 - kvp.Key))
        : */states.Values.Max(s => s.ReleasedPressure);

    bool IsValveOpen(Node node, int openState) => ((1 << node.OpenIndex) & openState) != 0;
}

Node[] ReadInput()
{
    var primes = new[]
    {
        2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61,
        67, 71, 73, 79, 83, 89, 97, 101, 103, 107, 109, 113, 127, 131, 137,
        139, 149, 151, 157, 163, 167, 173, 179, 181, 191, 193, 197, 199, 211,
        223, 227, 229, 233, 239, 241, 251, 257, 263, 269, 271, 277, 281, 283,
        293, 307, 311, 313, 317, 331, 337, 347, 349, 353, 359, 367, 373, 379,
        383, 389, 397, 401, 409, 419, 421, 431, 433, 439, 443, 449, 457, 461,
        463, 467, 479, 487, 491, 499, 503, 509, 521, 523, 541
    };

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
                    nodes[n] = nnode;
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
    var idIndex = 0;
    foreach (var node in nodes.Values.OrderBy(n => n.Name))
    {
        node.Id = primes[idIndex++];
        if (node.FlowRate > 0) node.OpenIndex = index++;
    }

    var result = new Node[primes[idIndex - 1] + 1];
    foreach (var node in nodes.Values)
    {
        result[node.Id] = node;
    }

    return result;
}

record struct State(int Id1, int Id2, int TotalRate, int ReleasedPressure);
record struct StateKey(int OpenNodes, int Current);

record Node(string Name, List<Node> Neighbours)
{
    public Node(string name, int flowRate, List<Node>? neighbours = null)
        : this(name, neighbours ?? new())
    {
        FlowRate = flowRate;
    }

    public int FlowRate { get; set; }
    public int OpenIndex { get; set; } = -1;
    public int Id { get; set; } = -1;
}

class Grid
{
}