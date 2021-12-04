using Slice = System.Collections.Generic.HashSet<int>;

class Grid
{
    private readonly List<Slice> _rows = new();
    private readonly Dictionary<int, List<Slice>> _numbersToRows = new();
    private readonly Dictionary<int, List<Slice>> _numbersToColumns = new();

    public Grid(IEnumerable<int[]> rows)
    {
        var cols = Enumerable.Range(0, 5).Select(_ => new Slice()).ToList();
        foreach (var (r_i, row) in rows.Enumerate())
        {
            var newRow = new Slice();
            _rows.Add(newRow);

            foreach (var (c_i, n) in row.Enumerate())
            {
                newRow.Add(n);
                AddToNumberDictionary(_numbersToRows, n, newRow);

                cols[c_i].Add(n);
                AddToNumberDictionary(_numbersToColumns, n, cols[c_i]);

            }
        }
    }
    public int Punch(int n)
    {
        // Remove n from matching rows, and then columns
        if (PunchThrough(_numbersToRows, n) || PunchThrough(_numbersToColumns, n))
        { 
            return n * _rows.SelectMany(r => r).Sum();
        }

        return 0;
    }

    private void AddToNumberDictionary(Dictionary<int, List<Slice>> dict, int n, Slice entry)
    {
        if (!dict.TryGetValue(n, out var indexes))
        {
            indexes = new();
            dict[n] = indexes;
        }

        indexes.Add(entry);
    }

    private bool PunchThrough(Dictionary<int, List<Slice>> groupsByNumber, int n)
    {
        if (groupsByNumber.TryGetValue(n, out var groups))
        {
            foreach (var group in groups)
            {
                group.Remove(n);
                if (group.Count == 0)
                {
                    return true;
                }
            }
        }

        return false;
    }
}
