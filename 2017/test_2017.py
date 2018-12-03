from test_utils import get_test_runner_for_year

challenge_test = get_test_runner_for_year(2017)

def test_day01():
    challenge_test(1,
        1216,
        1072
    )

def test_day02():
    challenge_test(2,
        43074,
        280,
    )

def test_day03():
    pass

def test_day04():
    challenge_test(4,
        337,
        231,
    )

def test_day05():
    challenge_test(5,
        373160,
        26395586,
    )
