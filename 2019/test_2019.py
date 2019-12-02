from test_utils import get_test_runner_for_year
import pytest

challenge_test = get_test_runner_for_year(2019)

def test_day01():
    challenge_test(1,
        3262356,
        4890664
    )

def test_day02():
    challenge_test(2,
        3058646,
        8976
    )
