using System.Diagnostics;
using System.Globalization;

var input = ParseInput();
var parser = new Parser(input);
var packet = parser.ParsePacket(out _);

Test("C200B40A82");
Test("04005AC33890");
Test("880086C3E88112");
Test("CE00C43D881120");
Test("D8005AC2A8F0");
Test("F600BC2D8F");
Test("9C005AC2F8F0");
Test("9C0141080250320F1802104A08");
Console.WriteLine("Part 1: {0}", GetVersionNumberSums(packet));
Console.WriteLine("Part 2: {0}", Evaluate(packet));

void Test(string input)
{
    var parser = new Parser(input);
    var packet = parser.ParsePacket(out _);
    Console.WriteLine($"{input}: {Evaluate(packet)}");
}

long Evaluate(Packet packet) =>
    packet switch
    {
        LiteralValuePacket(_, _, var value) => value,
        OperatorPacket(_, var id, var subPackets) =>
            id switch
            {
                0 => subPackets.Sum(Evaluate),
                1 => subPackets.Aggregate(1L, (product, p) => product * Evaluate(p)),
                2 => subPackets.Min(Evaluate),
                3 => subPackets.Max(Evaluate),
                5 => Evaluate(subPackets[0]) > Evaluate(subPackets[1]) ? 1 : 0,
                6 => Evaluate(subPackets[0]) < Evaluate(subPackets[1]) ? 1 : 0,
                7 => Evaluate(subPackets[0]) == Evaluate(subPackets[1]) ? 1 : 0,
                _ => throw new NotImplementedException(),
            },
        _ => throw new InvalidOperationException("???"),
    };

int GetVersionNumberSums(Packet packet) =>
    packet switch
    {
        LiteralValuePacket(var version, _, _) => version,
        OperatorPacket(var version, _, var subPackets) => version + subPackets.Sum(p => GetVersionNumberSums(p)),
        _ => throw new InvalidOperationException("???"),
    };

string ParseInput()
{
    return File.ReadAllText(args[0]).Trim();
}

abstract record Packet(int Version, int TypeId);
record LiteralValuePacket(int Version, int TypeId, long Value) : Packet(Version, TypeId)
{
    public override string ToString()
    {
        return Value.ToString();
    }
}

record OperatorPacket(int Version, int TypeId, List<Packet> SubPackets) : Packet(Version, TypeId)
{
    public override string ToString()
    {
        return TypeId switch
        {
            0 => $"({string.Join(" + ", SubPackets.Select(p => p.ToString()))})",
            1 => $"({string.Join(" * ", SubPackets.Select(p => p.ToString()))})",
            2 => $"Min({string.Join(", ", SubPackets.Select(p => p.ToString()))}",
            3 => $"Max({string.Join(", ", SubPackets.Select(p => p.ToString()))}",
            5 => $"{SubPackets[0]} > {SubPackets[1]}",
            6 => $"{SubPackets[0]} < {SubPackets[1]}",
            7 => $"{SubPackets[0]} = {SubPackets[1]}",
            _ => throw new NotImplementedException(),
        };
    }
}

class Parser
{
    private readonly uint[] _dwords;
    private int _offset;

    public Parser(string input)
    {
        _dwords = input
            .Buffer(8)
            .Select(digits => digits
                .Aggregate(0u, (n, d) => n * 16 + uint.Parse(d.ToString(), NumberStyles.HexNumber)) << 4*(8 - digits.Count))
            .ToArray();
    }

    public Packet ParsePacket(out int readBits)
    {
        var packageVersion = ReadNext(3);
        var packageTypeId = ReadNext(3);

        if (packageTypeId == 4) return ParseLiteralValuePacket(packageVersion, packageTypeId, out readBits);
        return ParseOperatorPacket(packageVersion, packageTypeId, out readBits);
    }

    private Packet ParseOperatorPacket(int packageVersion, int packageTypeId, out int readBits)
    {
        var lengthTypeId = ReadNext(1);
        readBits = 7;
        var subPackets = new List<Packet>();
        
        if (lengthTypeId == 0)
        {
            var subPacketsBits = 0;
            // Read until the length in bit is reached
            var totalBitLength = ReadNext(15);
            readBits += 15;
            while (subPacketsBits < totalBitLength)
            {
                subPackets.Add(ParsePacket(out var packetBits));
                subPacketsBits += packetBits;
            }
            Debug.Assert(subPacketsBits == totalBitLength);
            readBits += subPacketsBits;
        }
        else
        {
            var packetCount = ReadNext(11);
            readBits += 11;
            for (int i = 0; i < packetCount; i++)
            {
                subPackets.Add(ParsePacket(out var packetBits));
                readBits += packetBits;
            }
        }

        return new OperatorPacket(packageVersion, packageTypeId, subPackets);
    }

    private Packet ParseLiteralValuePacket(int packageVersion, int packageTypeId, out int readBits)
    {
        readBits = 6;
        var number = 0L;
        while (true)
        {
            var numberPart = ReadNext(5);
            readBits += 5;
            number = (number << 4) + (numberPart & 0xF);
            if ((numberPart & 0x10) == 0)
                break;
        }
        return new LiteralValuePacket(packageVersion, packageTypeId, number);
    }

    public int ReadNext(int bitLength)
    {
        if (bitLength > 32) throw new InvalidOperationException($"Nope! Bitlength too long: {bitLength}");

        var result = 0;
        while (bitLength > 0)
        {
            var index = _offset / 32;
            var blockBits = _dwords[index];
            var dwordOffset = _offset % 32;
            blockBits <<= dwordOffset;

            //if (bitLength <= 32 - dwordOffset)
            //{
            // |-----32-----|
            // |    o*******|
            // |********
            // |--bl--|     | 
            var readBitCount = Math.Min(bitLength, 32 - dwordOffset);
            blockBits >>= 32 - readBitCount;
            _offset += readBitCount;
            result = (result << readBitCount) + (int)blockBits;
            bitLength -= readBitCount;
        }

        return result;

    }
}
