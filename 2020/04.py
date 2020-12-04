from utils import *

required_fields = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]

def is_int_between(value, lower, upper):
    if not is_int(value): return False
    return lower <= int(value) <= upper

@check(260)
def part1(passports):
    def is_valid(passport):
        return all(field in passport for field in required_fields)

    return len(list(filter(is_valid, passports)))

@check(153)
def part2(passports):
    def validate_field(field, value):
        if field == "byr":
            return is_int_between(value, 1920, 2002)
        elif field == "iyr":
            return is_int_between(value, 2010, 2020)
        elif field == "eyr":
            return is_int_between(value, 2020, 2030)
        elif field == "hgt":
            if value.endswith("cm"):
                value = value[:-2]
                return is_int_between(value, 150, 193)
            elif value.endswith("in"):
                value = value[:-2]
                return is_int_between(value, 59, 76)
            else:
                return False
        elif field == "hcl":
            if not value.startswith("#"): return False
            value = value[1:]
            return re.match("[0-9a-f]*", value)
        elif field == "ecl":
            return value in ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"]
        elif field == "pid":
            return len(value) == 9 and is_int(value)

    def is_valid(passport):
        return all(field in passport and validate_field(field, passport[field]) for field in required_fields)

    return len(list(filter(is_valid, passports)))

def transform_input(i):
    def get_fields(line):
        fields = line.split()
        fields = [f.split(":") for f in fields]
        fields = { f[0]: f[1] for f in fields }
        return fields

    i = i.split("\n\n")
    i = [fields.replace("\n", " ") for fields in i]
    i = [get_fields(fields) for fields in i]
    return i
