using System.Text.RegularExpressions;

RunAndMeasureTime("Part1", Part1);
RunAndMeasureTime("Part2", Part2);

int Part1()
{
    var blueprints = ReadInput();

    return Solve(blueprints, 24);
}

int Part2()
{
    return 0;
}

int Solve(Blueprint[] blueprints, int minutes)
{
    int GetGeodeCount(Blueprint blueprint)
    {
        var states = new List<State> { new() { OreRobots = 1 } };
        for (int i = 0; i < minutes; i++)
        {
            var newStates = new List<State>(states.Count);
            foreach (var state in states)
            {
                var (oreRobots, clayRobots, obsidianRobots, geodeRobots) = (state.OreRobots, state.ClayRobots, state.ObsidianRobots, state.GeodeRobots);
                if (state.Ore >= blueprint.geodeRobotCost.ore && state.Obsidian >= blueprint.geodeRobotCost.obsidian)
                {
                    state.Ore -= blueprint.geodeRobotCost.ore;
                    state.Obsidian -= blueprint.geodeRobotCost.obsidian;
                    state.GeodeRobots++;
                }

                if (state.Ore >= blueprint.oreRobotCost)
                {
                    state.Ore -= blueprint.oreRobotCost;
                    state.OreRobots++;
                }

                state.Geodes += geodeRobots;
                state.Obsidian += obsidianRobots;
                state.Clay += clayRobots;
                state.Ore += oreRobots;


            }
        }

        return states.Max(s => s.Geodes);
    }

    return blueprints.Max(GetGeodeCount);
}

Blueprint[] ReadInput()
{
    return ReadLinesFromInputFile(args)
        .Select(ParseBluePrint)
        .ToArray();

    Blueprint ParseBluePrint(string line)
    {
        var m = Regex.Match(line,
            @"Each ore robot costs (\d+) ore. Each clay robot costs (\d+) ore. Each obsidian robot costs (\d+) ore and (\d+) clay. Each geode robot costs (\d+) ore and (\d+) obsidian.");
        if (!m.Success) throw new Exception("Invalid string: " + line);
        return new(
            int.Parse(m.Groups[1].Value),
            int.Parse(m.Groups[2].Value),
            (int.Parse(m.Groups[3].Value), int.Parse(m.Groups[4].Value)),
            (int.Parse(m.Groups[5].Value), int.Parse(m.Groups[6].Value))
            );
    }
}

record Blueprint(int oreRobotCost, int clayRobotCost, (int ore, int clay) obsidianRobotCost, (int ore, int obsidian) geodeRobotCost);
record State
{
    public bool CanProduceOreRobot(Blueprint bp) => Ore >= bp.oreRobotCost;
    public bool CanProduceClayRobot(Blueprint bp) => Ore >= bp.clayRobotCost;
    public bool CanProduceObsidianRobot(Blueprint bp) => Ore >= bp.obsidianRobotCost.ore && Clay >= bp.obsidianRobotCost.clay;
    public bool CanProduceGeodeRobot(Blueprint bp) => Obsidian >= bp.geodeRobotCost.obsidian && Ore >= bp.geodeRobotCost.ore;

    public int Ore { get; set; }
    public int Clay { get; set; }
    public int Obsidian { get; set; }
    public int Geodes { get; set; }

    public int OreRobots { get; set; } = 1;
    public int ClayRobots { get; set; }
    public int ObsidianRobots { get; set; }
    public int GeodeRobots { get; set; }
}