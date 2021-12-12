using System.Collections.Immutable;
using static Graph;

var graph = ParseLines();
var paths = GetPathsToEnd(graph.GetNode("start"), "", ImmutableHashSet.Create<string>());
Console.WriteLine("Part 1: {0}", paths.Count);

paths = GetPathsToEnd2(graph.GetNode("start"), "", ImmutableHashSet.Create<string>(), null);
Console.WriteLine("Part 2: {0}", paths.Count);


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

ImmutableHashSet<string> GetPathsToEnd(Node fromNode, string path, ImmutableHashSet<string> visitedMinorNodes)
{
    path += fromNode.Name;
    //Console.WriteLine(path);
    if (fromNode.Name == "end") return ImmutableHashSet.Create(path);

    var paths = ImmutableHashSet.Create<string>();
    if (!fromNode.IsMajor) visitedMinorNodes = visitedMinorNodes.Add(fromNode.Name);

    foreach (var nextNode in fromNode.Links)
    {
        if (!nextNode.IsMajor && visitedMinorNodes.Contains(nextNode.Name)) continue;
        paths = paths.Union(GetPathsToEnd(nextNode, path, visitedMinorNodes));
    }

    return paths;
}

ImmutableHashSet<string> GetPathsToEnd2(Node fromNode, string path, ImmutableHashSet<string> visitedMinorNodes, string? twiceVisitedMinor)
{
    path += fromNode.Name;
    //Console.WriteLine(path);
    if (fromNode.Name == "end") return ImmutableHashSet.Create(path);

    var paths = ImmutableHashSet.Create<string>();
    if (!fromNode.IsMajor) visitedMinorNodes = visitedMinorNodes.Add(fromNode.Name);

    foreach (var nextNode in fromNode.Links)
    {
        var newTwiceVisitedMinor = twiceVisitedMinor;
        if (!nextNode.IsMajor && visitedMinorNodes.Contains(nextNode.Name))
        {
            if (nextNode.Name == "start" || twiceVisitedMinor != null) continue;
            newTwiceVisitedMinor = nextNode.Name;

        }
        paths = paths.Union(GetPathsToEnd2(nextNode, path, visitedMinorNodes, newTwiceVisitedMinor));
    }

    return paths;
}

class Graph
{
    private Dictionary<string, Node> _nodes = new();
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