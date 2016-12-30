import unittest

from test_utils import Part1, Part2

@Part1(1, 241)
@Part2(1, 116)
class TestDay1(unittest.TestCase): pass

@Part1(2, "35749")
@Part2(2, "9365C")
class TestDay2(unittest.TestCase): pass

@Part1(3, 993)
@Part2(3, 1849)
class TestDay3(unittest.TestCase): pass

@Part1(4, 173787)
@Part2(4, 548)
class TestDay4(unittest.TestCase): pass

@Part1(5, "4543c154", skip=True)
@Part2(5, "1050cbbd", skip=True)
class TestDay5(unittest.TestCase): pass

@Part1(6, "afwlyyyq")
@Part2(6, "bhkzekao")
class TestDay6(unittest.TestCase): pass

@Part1(7, 110)
@Part2(7, 242)
class TestDay7(unittest.TestCase): pass

@Part1(8, 121)
@Part2(8,
"""###  #  # ###  #  #  ##  ####  ##  ####  ### #    
#  # #  # #  # #  # #  # #    #  # #      #  #    
#  # #  # #  # #  # #    ###  #  # ###    #  #    
###  #  # ###  #  # #    #    #  # #      #  #    
# #  #  # # #  #  # #  # #    #  # #      #  #    
#  #  ##  #  #  ##   ##  ####  ##  ####  ### #### 
""")
class TestDay8(unittest.TestCase): pass

@Part1(9, 120765, skip=True)
@Part2(9, 11658395076, skip=True)
class TestDay9(unittest.TestCase): pass

@Part1(10, 86)
@Part2(10, 22847)
class TestDay10(unittest.TestCase): pass

@Part1(11, 33)
@Part2(11, 57)
class TestDay11(unittest.TestCase): pass

@Part1(12, 318117, skip=True)
@Part2(12, 9227771, skip=True)
class TestDay12(unittest.TestCase): pass

@Part1(13, 90)
@Part2(13, 135)
class TestDay13(unittest.TestCase): pass

@Part1(14, 23769, skip=True)
@Part2(14, 20606, skip=True)
class TestDay14(unittest.TestCase): pass

@Part1(15, 148737)
@Part2(15, 2353212)
class TestDay15(unittest.TestCase): pass

@Part1(16, "10010010110011010")
@Part2(16, "01010100101011100")
class TestDay16(unittest.TestCase): pass

@Part1(17, "RDDRULDDRR")
@Part2(17, 766)
class TestDay17(unittest.TestCase): pass

@Part1(18, 1982)
@Part2(18, 20005203)
class TestDay18(unittest.TestCase): pass

@Part1(19, 1816276)
@Part2(19, 1410967)
class TestDay19(unittest.TestCase): pass

@Part1(20, 17348574)
@Part2(20, 104)
class TestDay20(unittest.TestCase): pass

if __name__ == "__main__":
    unittest.main(verbosity=2)
