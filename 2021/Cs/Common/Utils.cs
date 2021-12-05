public class Utils
{
    public static StreamReader OpenInputFileAsStream(string[] args)
    {
        if (args.Length < 1)
        {
            Console.WriteLine("Missing file name");
            Environment.Exit(1);
        }

        return File.OpenText(args[0]) ?? throw new Exception($"Failed to open file {args[1]}");
    }
}
