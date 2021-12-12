using System.Collections.Immutable;
using static Graph;

var graph = ParseLines();
var count = GetPathsToEnd(graph.GetNode("start"), ImmutableHashSet.Create<string>());
Console.WriteLine("Part 1: {0}", count);

count = GetPathsToEnd2(graph.GetNode("start"), ImmutableHashSet.Create<string>(), null);
Console.WriteLine("Part 2: {0}", count);


Graph ParseLines()
{
    var graph = new Graph();
    foreach (var line in Utils.ReadLinesFromInputFile(args))
    {
        var parsed = line.Split('-');
        graph.AddLink(parsed[0], parsed[1]);
    }
    return graph;
}

int GetPathsToEnd(Node fromNode, ImmutableHashSet<string> visitedMinorNodes)
{
    if (fromNode.Name == "end") return 1;

    var paths = ImmutableHashSet.Create<string>();
    if (!fromNode.IsMajor) visitedMinorNodes = visitedMinorNodes.Add(fromNode.Name);

    var count = 0;
    foreach (var nextNode in fromNode.Links)
    {
        if (!nextNode.IsMajor && visitedMinorNodes.Contains(nextNode.Name)) continue;
        count += GetPathsToEnd(nextNode, visitedMinorNodes);
    }

    return count;
}

int GetPathsToEnd2(Node fromNode, ImmutableHashSet<string> visitedMinorNodes, string? twiceVisitedMinor)
{
    if (fromNode.Name == "end") return 1;

    if (!fromNode.IsMajor) visitedMinorNodes = visitedMinorNodes.Add(fromNode.Name);

    var count = 0;
    foreach (var nextNode in fromNode.Links)
    {
        var newTwiceVisitedMinor = twiceVisitedMinor;
        if (!nextNode.IsMajor && visitedMinorNodes.Contains(nextNode.Name))
        {
            if (nextNode.Name == "start" || twiceVisitedMinor != null) continue;
            newTwiceVisitedMinor = nextNode.Name;

        }
        count += GetPathsToEnd2(nextNode, visitedMinorNodes, newTwiceVisitedMinor);
    }

    return count;
}

class Graph
{
    private readonly Dictionary<string, Node> _nodes = new();
    public class Node
    {
        public string Name { get; }
        public bool IsMajor { get; }

        public Node(string name)
        {
            Name = name;
            IsMajor = char.IsUpper(name[0]);
        }

        public List<Node> Links = new();
    }

    public void AddLink(string n1, string n2)
    {
        var node1 = GetNode(n1);
        var node2 = GetNode(n2);
        node1.Links.Add(node2);
        node2.Links.Add(node1);
    }

    public Node GetNode(string name)
    {
        if (!_nodes.TryGetValue(name, out var node))
        {
            node = new Node(name);
            _nodes.Add(name, node);
        }

        return node;
    }
}