from utils import *
import heapq

@check(1586300)
def part1(presents):
    paper_area = sum(paper_area_required(present) for present in presents)
    return paper_area

def paper_area_required(present):
    (l, w, h) = present
    sides = (l*w, w*h, h*l)
    total_area = sum(map(lambda s: s*2, sides))
    extra_paper = min(sides)
    return total_area + extra_paper

@check(3737498)
def part2(presents):
    ribbon_length = sum(ribbon_length_required(present) for present in presents)
    return ribbon_length

def ribbon_length_required(present):
    two_smallest_sides = heapq.nsmallest(2, present)
    smallest_perimeter = 2 * sum(two_smallest_sides)

    present_volume = reduce(operator.mul, present, 1)

    return smallest_perimeter + present_volume

def transform_input(challenge_input):
    def parse_line(line):
        match = re.search("^(\d+)x(\d+)x(\d+)$", line)
        length = int(match.group(1))
        width  = int(match.group(2))
        height = int(match.group(3))
        return (length, width, height)

    presents = [parse_line(line) for line in challenge_input.splitlines()]
    return presents
