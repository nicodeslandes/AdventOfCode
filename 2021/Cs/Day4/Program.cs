using Row = System.Collections.Generic.HashSet<int>;
using Col = System.Collections.Generic.HashSet<int>;

if (args.Length < 1)
{
    Console.WriteLine("Missing file name");
    Environment.Exit(1);
}

using var f = File.OpenText(args[0]) ?? throw new Exception($"Failed to open file {args[1]}");
var numbers = (await f.ReadLineAsync())!.Split(',').Select(x => int.Parse(x)).ToList();
Console.WriteLine("Numbers: {0}", string.Join(",", numbers));

IEnumerable<string> GetLines()
{
    string? line;
    while ((line = f!.ReadLine()) != null)
    {
        yield return line;
    }
}

var grids = GetLines()
    .Buffer(6)
    .Select(b => b
        .Skip(1)
        .Select(l => l.Split(' ', StringSplitOptions.RemoveEmptyEntries).Select(int.Parse).ToArray())
        )
    .Select(rows => new Grid(rows))
    .ToArray();

int PunchAllGridsUntilFirstWin()
{
    foreach (var n in numbers)
    {
        foreach (var g in grids)
        {
            var result = g.Punch(n);
            if (result != 0) return result;
        }
    }
    return 0;
}

int PunchAllGridsUntilNoGridLeft()
{
    var gridsLeft = Enumerable.Range(0, grids.Length).ToHashSet();
    foreach (var n in numbers)
    {
        foreach (var (g_i, g) in grids.Enumerate())
        {
            var result = g.Punch(n);
            if (result != 0
                && gridsLeft.Remove(g_i)
                && gridsLeft.Count == 0)
            {
                return result;
            }
        }
    }
    return 0;
}



Console.WriteLine("Part 1: {0}", PunchAllGridsUntilFirstWin());
Console.WriteLine("Part 2: {0}", PunchAllGridsUntilNoGridLeft());


class Grid
{
    private List<Row> _rows = new();
    private List<Col> _cols = new();
    private Dictionary<int, List<Row>> _numbersToRows = new();
    private Dictionary<int, List<Col>> _numbersToColumns = new();

    public Grid(IEnumerable<int[]> rows)
    {
        _cols = Enumerable.Range(0, 5).Select(_ => new Col()).ToList();
        foreach (var (r_i, row) in rows.Enumerate())
        {
            var newRow = new Row();
            _rows.Add(newRow);
            foreach (var (c_i, n) in row.Enumerate())
            {
                newRow.Add(n);
                _cols[c_i].Add(n);
                if (!_numbersToColumns.TryGetValue(n, out var nbCols))
                {
                    nbCols = new();
                    _numbersToColumns[n] = nbCols;
                }

                nbCols.Add(_cols[c_i]);

                if (!_numbersToRows.TryGetValue(n, out var nbRows))
                {
                    nbRows = new();
                    _numbersToRows[n] = nbRows;
                }

                nbRows.Add(newRow);
            }
        }
    }

    public int Punch(int n)
    {
        var won = false;

        // Remove n from row
        if (_numbersToRows.TryGetValue(n, out var rows))
        {
            foreach (var row in rows)
            {
                row.Remove(n);
                if (row.Count == 0)
                {
                    won = true;
                }
            }

            if (won)
            {
                return n * _rows.SelectMany(r => r).Sum();
            }
        }

        if (_numbersToColumns.TryGetValue(n, out var cols))
        {
            foreach (var col in cols)
            {
                col.Remove(n);
                if (col.Count == 0)
                {
                    won = true;
                }
            }

            if (won)
            {
                return n * _rows.SelectMany(r => r).Sum();
            }
        }

        return 0;
    }
}

public static class EnumerableExt
{
    public static IEnumerable<(int index, T value)> Enumerate<T>(this IEnumerable<T> coll)
        => coll.Select((val, i) => (i, val));
}