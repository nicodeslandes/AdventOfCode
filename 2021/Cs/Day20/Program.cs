using System.Text.RegularExpressions;

var (algorithm, grid) = ParseInput();

grid.Print();
Console.WriteLine("Enhancing");
grid.Enhance(algorithm);
grid.Enhance(algorithm);
grid.Print();
var part1 = grid.CountLitPixels();
Console.WriteLine("Part 1: {0}", part1);


(int[] algorigthm, Grid grid) ParseInput()
{
    var lines = File.ReadLines(args[0]).ToList();
    var algorithm = lines[0].Select(ch => ch == '.' ? 0 : 1).ToArray();

    var grid = new Grid(lines.Skip(2));
    return (algorithm, grid);
}

class Grid
{
    private int[][] _values;
    private int _blankValue = 0;

    public Grid(IEnumerable<string> rows)
    {
        _values = rows.Select(row => row.Select(ch => ch == '.' ? 0 : 1).ToArray()).ToArray();
    }

    public void Enhance(int[] algorithm)
    {
        var newGrid = new int[_values.Length + 2][];

        for (int y = 0; y < _values.Length + 2; y++)
        {
            newGrid[y] = new int[_values[0].Length + 2];
            for (int x = 0; x < _values[0].Length + 2; x++)
            {
                var (sx, sy) = (x - 1, y - 1);
                int GetValue(int vx, int vy)
                {
                    if (vx < 0 || vx >= _values[0].Length) return _blankValue;
                    if (vy < 0 || vy >= _values.Length) return _blankValue;
                    return _values[vy][vx];
                }
                int index = 0;
                for (var dy = -1; dy <= 1; dy++)
                {
                    for (var dx = -1; dx <= 1; dx++)
                    {
                        index = (index << 1) + GetValue(sx + dx, sy + dy);
                    }
                }

                newGrid[y][x] = algorithm[index];
            }
        }

        _values = newGrid;
        _blankValue = algorithm[_blankValue == 0 ? 0 : 511];
    }

    public void Print()
    {
        foreach (var row in _values)
        {
            Console.WriteLine(string.Join("", row.Select(x => x == 0 ? '.' : '#')));
        }
    }

    public int CountLitPixels() => _values.SelectMany(r => r).Sum();
}

record Pos(int X, int Y);
