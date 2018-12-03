from test_utils import get_test_runner_for_year
import pytest

challenge_test = get_test_runner_for_year(2016)

def test_day01():
    challenge_test(1,
        241,
        116
    )

def test_day02():
    challenge_test(2,
        "35749",
        "9365C"
    )

def test_day03():
    challenge_test(3,
        993,
        1849
    )

def test_day04():
    challenge_test(4,
        173787,
        548
    )

@pytest.mark.skip(reason="Slow!")
def test_day05():
    challenge_test(5,
        "4543c154",
        "1050cbbd",
    )

def test_day06():
    challenge_test(6,
        "afwlyyyq",
        "bhkzekao"
    )

def test_day07():
    challenge_test(7,
        110,
        242
    )

def test_day08():
    challenge_test(8,
        121,
"""###  #  # ###  #  #  ##  ####  ##  ####  ### #    
#  # #  # #  # #  # #  # #    #  # #      #  #    
#  # #  # #  # #  # #    ###  #  # ###    #  #    
###  #  # ###  #  # #    #    #  # #      #  #    
# #  #  # # #  #  # #  # #    #  # #      #  #    
#  #  ##  #  #  ##   ##  ####  ##  ####  ### #### 
"""
    )

def test_day09():
    challenge_test(9,
        120765,
        11658395076,
    )

def test_day10():
    challenge_test(10,
        86,
        22847
    )

def test_day11():
    challenge_test(11,
        33,
        57
    )

@pytest.mark.skip(reason="Slow!")
def test_day12():
    challenge_test(12,
        318117,
        9227771,
    )

def test_day13():
    challenge_test(13,
        90,
        135
    )

@pytest.mark.skip(reason="Slow!")
def test_day14():
    challenge_test(14,
        23769,
        20606,
    )

def test_day15():
    challenge_test(15,
        148737,
        2353212
    )

def test_day16():
    challenge_test(16,
        "10010010110011010",
        "01010100101011100"
    )

def test_day17():
    challenge_test(17,
        "RDDRULDDRR",
        766
    )

def test_day18():
    challenge_test(18,
        1982,
        20005203
    )

def test_day19():
    challenge_test(19,
        1816276,
        1410967
    )

def test_day20():
    challenge_test(20,
        17348574,
        104
    )

def test_day21():
    challenge_test(21,
        "gfdhebac",
        "dhaegfbc"
    )

def test_day22():
    challenge_test(22,
        901,
        238
    )

@pytest.mark.skip(reason="Slow!")
def test_day23():
    challenge_test(23,
        12703,
        479009263,
    )

def test_day24():
    challenge_test(24,
        442,
        660
    )

def test_day25():
    challenge_test(25,
        198,
        None
    )
