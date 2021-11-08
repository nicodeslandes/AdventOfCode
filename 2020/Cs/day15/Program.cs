using System;
using System.Diagnostics;

var StartingNumbers = new[] {0,3,6};
long get_nth(int[] start, int nth)
{
	var numbers_turns = new ulong[nth];
	var i = 0;
	for (; i < nth; i++) numbers_turns[i] = 0xFFFFFFFFFFFFFFFF;


	for (i = 0; i < start.Length; i++)
		numbers_turns[start[i]] = 0xFFFFFFFF00000000 | (ulong)i;

	ulong last = (ulong)start[^1];
	for (; i < nth; i++)
	{
		var turns = numbers_turns[last];
		var first = turns >> 32;
		var second = turns & 0xFFFFFFFF;
		
		var value = first == 0xFFFFFFFF ? 0 : second - first;
		numbers_turns[value] = (numbers_turns[value] << 32) | (ulong)i;

		last = value;
	}

	return (int)last;
}

long get_nth2(int[] start, int nth)
{
	var numbers_turns = new ulong[nth + 1];
	for (int i = 0; i < start.Length; i++)
	{
		var val = (ulong)(i + 1);
		numbers_turns[start[i]] = val | (val << 32);
	}

	ulong lastSpoken = (ulong)start[^1];
	for (ulong i = (ulong)start.Length + 1; i <= (ulong)nth; i++)
	{
		var turns = numbers_turns[lastSpoken];
		var last = turns >> 32;
		var prev = turns & 0xFFFFFFFF;

		lastSpoken = last - prev;
		last = numbers_turns[lastSpoken] >> 32;
		numbers_turns[lastSpoken] = (i << 32) | (last == 0 ? i : last);
	}

	return (int)lastSpoken;
}


int SolveForTurn(int finalTurn)
{
    // Unreadable code. But I was having fun optimizing for speed :)
    var spokenNumbers = new (int last, int prev)[finalTurn+1];

    for (var i = 0; i < StartingNumbers.Length; i++)
        spokenNumbers[StartingNumbers[i]] = (i + 1, i + 1);

    var lastSpokenNumber = StartingNumbers[^1];
    for (var turn = StartingNumbers.Length + 1; turn <= finalTurn; turn++)
    {
        var (last, prev) = spokenNumbers[lastSpokenNumber];
        lastSpokenNumber = last - prev;

        (last, _) = spokenNumbers[lastSpokenNumber];
        spokenNumbers[lastSpokenNumber] = last == 0 ? (turn, turn) : (turn, last);
    }

    return lastSpokenNumber;
}


long part1(params int[] start)
{
	return get_nth(start, 2020);
}


long part2_1(int iter, params int[] start)
{
	long res = 0;
	for (int i = 0; i < iter; i++)
		res = get_nth(start, 30_000_000);
	return res;
}

long part2_2(int iter, params int[] start)
{
	long res = 0;
	for (int i = 0; i < iter; i++)
		res = get_nth2(start, 30_000_000);
	return res;
}

long part2_3(int iter)
{
	long res = 0;
	for (int i = 0; i < iter; i++)
		res = SolveForTurn(30_000_000);
	return res;
}

part2_1(1, 0, 3, 6);
part2_2(1, 0, 3, 6);
part2_3(1);
GC.Collect();
GC.WaitForPendingFinalizers();
GC.WaitForFullGCComplete();
GC.Collect();
GC.WaitForFullGCComplete();
var sw = Stopwatch.StartNew();
Console.WriteLine("Result: {0}", part2_1(15, 0, 3, 6));
Console.WriteLine("Elapsed: {0:N0} ms", sw.ElapsedMilliseconds/15);

GC.Collect();
GC.WaitForPendingFinalizers();
GC.WaitForFullGCComplete();
GC.Collect();
GC.WaitForFullGCComplete();
sw = Stopwatch.StartNew();
Console.WriteLine("Result: {0}", part2_2(15, 0, 3, 6));
Console.WriteLine("Elapsed: {0:N0} ms", sw.ElapsedMilliseconds/15);

GC.Collect();
GC.WaitForPendingFinalizers();
GC.WaitForFullGCComplete();
GC.Collect();
GC.WaitForFullGCComplete();
sw = Stopwatch.StartNew();
Console.WriteLine("Result: {0}", part2_3(15));
Console.WriteLine("Elapsed: {0:N0} ms", sw.ElapsedMilliseconds/15);