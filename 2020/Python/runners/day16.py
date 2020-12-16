from runners.utils import product
from typing import List
from logging import debug, info


def part1(input: List[str]) -> int:
    fields = {}
    my_ticket = []
    nearby_tickets = []
    scan_mode = 0
    for line in input:
        if line == '':
            continue
        if line == 'your ticket:':
            scan_mode = 1
            continue
        if line == 'nearby tickets:':
            scan_mode = 2
            continue
        if scan_mode == 0:
            [field_name, values] = line.split(': ')
            ranges = [(int(min_val), int(max_val))
                      for val_range in values.split(' or ')
                      for [min_val, max_val] in [val_range.split('-')]]
            fields[field_name] = ranges

        elif scan_mode == 1:
            my_ticket = map(int, line.split(','))
        elif scan_mode == 2:
            nearby_tickets.append(map(int, line.split(',')))
        else:
            raise Exception("What !?")

    debug("Fields: %s", fields)
    debug("Mine: %s", list(my_ticket))
    debug("Nearby: %s", list(nearby_tickets))

    def is_valid(value):
        def is_valid_for_field(f):
            r = any(r[0] <= value and r[1] >= value for r in f)
            debug("Is %d valid for field %s: %s", value, f, r)
            return r

        r = any(is_valid_for_field(f) for f in fields.values())
        debug("is_valid(%d): %s", value, r)
        return r

    return sum(value for ticket in nearby_tickets for value in ticket if not is_valid(value))


def part2(input: List[str]) -> int:
    fields = {}
    my_ticket = []
    nearby_tickets = []
    scan_mode = 0
    for line in input:
        if line == '':
            continue
        if line == 'your ticket:':
            scan_mode = 1
            continue
        if line == 'nearby tickets:':
            scan_mode = 2
            continue
        if scan_mode == 0:
            [field_name, values] = line.split(': ')
            ranges = [(int(min_val), int(max_val))
                      for val_range in values.split(' or ')
                      for [min_val, max_val] in [val_range.split('-')]]
            fields[field_name] = ranges

        elif scan_mode == 1:
            my_ticket = list(map(int, line.split(',')))
        elif scan_mode == 2:
            nearby_tickets.append(list(map(int, line.split(','))))
        else:
            raise Exception("What !?")

    debug("Fields: %s", fields)
    debug("Mine: %s", list(my_ticket))
    debug("Nearby: %s", list(nearby_tickets))

    def is_valid_for_field(f, value):
        r = any(r[0] <= value and r[1] >= value for r in f)
        debug("Is %d valid for field %s: %s", value, f, r)
        return r

    def is_valid(value):
        r = any(is_valid_for_field(f, value) for f in fields.values())
        debug("is_valid(%d): %s", value, r)
        return r

    nearby_tickets = [ticket for ticket in nearby_tickets
                      if all(is_valid(value) for value in ticket)]

    known_fields = {}
    field_indexes = {}
    unknown_fields = fields.copy()
    prev_count = 0
    while any(unknown_fields):
        if len(unknown_fields) == 1:
            info("Only unknown field is %s", list(unknown_fields.keys())[0])
            break
        for i in range(len(nearby_tickets[0])):
            if i in known_fields:
                continue
            # Find field that matches ticket[i] for all tickets
            possible_fields = []
            for field_name, ranges in unknown_fields.items():
                if all(is_valid_for_field(ranges, ticket[i]) for ticket in nearby_tickets):
                    debug("Field %d could be %s", i, field_name)
                    possible_fields.append(field_name)

            if len(possible_fields) == 1:
                field_name = possible_fields[0]
                known_fields[i] = field_name
                field_indexes[field_name] = i
                info("Only possible field for index %d is %s", i, field_name)
                del unknown_fields[field_name]
                break

        info("Known fields: %s", known_fields)
        if prev_count == len(known_fields):
            info("Can find any more; left: %s", unknown_fields.keys())
            break
        prev_count = len(known_fields)

    return product(my_ticket[index] for field, index in field_indexes.items() if field.startswith('departure'))
