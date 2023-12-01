using Humanizer;
using System.Linq;
using System.Text.RegularExpressions;
using System.Threading.Channels;

Console.WriteLine("Part1: {0}", Part1());
Console.WriteLine("Part2: {0}", Part2());

int Part1()
{
    var input = ReadInput();
    return ComputeCalibration(input);
}

int Part2()
{
    var digits = Enumerable.Range(1, 9).Select(x => x.ToWords())
        .Select((numberStr, index) => (number: index + 1, numberStr)).ToArray();

    var strToDigits = digits.ToDictionary(d => d.numberStr, d => d.number);

    var regexItems = digits
            .Select(x => $"{x.numberStr}");

    var regex = new Regex($"({string.Join("|", regexItems)})");
    var backRegex = new Regex($"({string.Join("|", regexItems)})", RegexOptions.RightToLeft);
    var input = ReadInput();
    input = input
        .Select(line =>
        {
            var original = line;
            Match match;
            if ((match = regex.Match(line)).Success)
            {
                var reg = new Regex(match.Value);
                var newline = reg.Replace(line, strToDigits[match.Value].ToString(), 1);
                line = newline;
            }

            var first = line.First(char.IsDigit);

            line = original;
            if ((match = backRegex.Match(line)).Success)
            {
                var reg = new Regex(match.Value, RegexOptions.RightToLeft);
                var newline = reg.Replace(line, strToDigits[match.Value].ToString(), 1);
                line = newline;
            }
            var last = line.Last(char.IsDigit);

            return $"{first}{last}";
        }).ToArray();
    return ComputeCalibration(input);
}

IEnumerable<string> ReadInput()
{
    return File.ReadAllLines(args[0]);
}

static int ComputeCalibration(IEnumerable<string> input)
{
    return input
        .Select(line => line.Where(char.IsDigit).ToArray() switch
        {
        [var ch] => new char[] { ch, ch },
        [var ch1, .., var ch2] => [ch1, ch2],
            var x => throw new NotImplementedException(),
        })
        .Select(chars => new string(chars))
        .Select(int.Parse)
        //.Select(i => { Console.WriteLine(i); return i; })
        .Sum();
}