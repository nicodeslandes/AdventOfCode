Console.WriteLine("Part1: {0}", Part1());
Console.WriteLine("Part2: {0}", Part2());


int Part1()
{
    var directories = ParseDirectories(args);

    return directories.Values.Where(d => d.Size <= 100_000).Sum(d => d.Size);
}

int Part2()
{
    var directories = ParseDirectories(args);
    var occupied = directories["/"].Size;
    var required = 30_000_000 - (70_000_000 - occupied);
    return directories.Values.Where(d => d.Size >= required).Min(d => d.Size);
}

Dictionary<string, Directory> ParseDirectories(string[] args)
{
    var lines = ReadInput();
    var currentPath = new Stack<string>();

    var directories = new Dictionary<string, Directory>();

    Directory GetOrAddDirectory(string path)
    {
        if (!directories.TryGetValue(path, out var entry))
        {
            entry = new Directory(path, new());
            directories[path] = entry;
        }

        return entry;
    }

    for (var i = 0; i < lines.Length; i++)
    {
        var line = lines[i];
        if (line.StartsWith('$'))
        {
            if (line.StartsWith("$ cd"))
            {
                var name = line[5..];
                if (name == "..") currentPath.Pop();
                else currentPath.Push(name);
            }
            else
            {
                // Command: ls

                var pathname = string.Join('/', currentPath.Reverse());
                var currentFolder = GetOrAddDirectory(pathname);

                // read the content
                while (i < lines.Length - 1 && lines[i + 1][0] != '$')
                {
                    var item = lines[++i].Split(' ');
                    Entry entry = item[0] == "dir" ?
                        GetOrAddDirectory($"{pathname}/{item[1]}") : new File(item[1], int.Parse(item[0]));
                    currentFolder.content.Add(entry);
                }
            }
        }
    }

    int ComputeSize(Directory dir)
    {
        if (dir.Size < 0)
        {
            dir.Size = 0;
            foreach (var item in dir.content)
            {
                dir.Size += item switch
                {
                    File f => f.size,
                    Directory d => ComputeSize(d),
                    _ => throw new Exception("What??")
                };
            }
        }

        return dir.Size;
    }

    ComputeSize(directories["/"]);
    return directories;
}


string[] ReadInput()
{
    return System.IO.File.ReadAllLines(args[0]);
}

abstract record Entry(string Name);
record File(string Name, int size) : Entry(Name);
record Directory(string Name, List<Entry> content) : Entry(Name)
{
    public int Size { get; set; } = -1;
}