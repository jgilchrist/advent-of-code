from utils import read_input

def Part1(challenge_number, answer, skip=False):
    import unittest

    def test_part1(self):
        """Part 1"""
        challenge = __import__(str(challenge_number))
        challenge_input = read_input(challenge_number)
        result = challenge.part1(challenge_input)
        self.assertEqual(result, answer)

    def wrap(cls):
        cls.test_part1 = test_part1
        if skip: cls.test_part1 = unittest.skip("Skipped Part 1")(cls.test_part1)
        return cls

    return wrap

def Part2(challenge_number, answer, skip=False):
    import unittest

    def test_part2(self):
        """Part 2"""
        challenge = __import__(str(challenge_number))
        challenge_input = read_input(challenge_number)
        result = challenge.part2(challenge_input)
        self.assertEqual(result, answer)

    def wrap(cls):
        cls.test_part2 = test_part2
        if skip: cls.test_part2 = unittest.skip("Skipped Part 2")(cls.test_part2)
        return cls

    return wrap
