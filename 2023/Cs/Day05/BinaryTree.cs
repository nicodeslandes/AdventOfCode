using System.Diagnostics.CodeAnalysis;

static class BinaryTree
{
    public static BinaryTree<T> Create<T>(Comparer<T>? comparer = null)
        => new(comparer ?? Comparer<T>.Default);

    public enum VisitorDirection
    {
        Left,
        Right,
        Stop,
        Found
    }
}

class BinaryTree<T>(Comparer<T> comparer)
{
    record Node(T Value)
    {
        public Node? Left { get; set; }
        public Node? Right { get; set; }
    }

    Node? _root;

    public bool IsEmpty => _root is null;

    public void Add(T value)
    {
        _root = Add(value, _root);
    }
    public void AddRange(IEnumerable<T> values)
    {
        foreach (var item in values)
        {
            Add(item);
        }
    }

    private Node Add(T value, Node? node)
    {
        if (node == null) return new(value);

        var cmp = comparer.Compare(value, node.Value);
        if (cmp < 0)
        {
            node.Left = Add(value, node.Left);
        }
        else if (cmp > 0)
        {
            node.Right = Add(value, node.Right);
        }

        return node;
    }

    public bool TryGetMatch(Func<T, BinaryTree.VisitorDirection> search, [MaybeNullWhen(false)] out T value)
    {
        var node = _root;
        while (node != null)
        {
            switch (search(node.Value))
            {
                case BinaryTree.VisitorDirection.Left:
                    node = node.Left;
                    break;
                case BinaryTree.VisitorDirection.Right:
                    node = node.Right;
                    break;
                case BinaryTree.VisitorDirection.Stop:
                    value = default;
                    return false;
                case BinaryTree.VisitorDirection.Found:
                    value = node.Value;
                    return true;
            }
        }

        value = default;
        return false;
    }
}
