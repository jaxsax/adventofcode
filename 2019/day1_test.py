import unittest
from day1 import fuel_required, fuel_required_until_empty, fuel_required_nofloor


class Day1Test(unittest.TestCase):
    def test_stage1(self):
        self.assertEqual(fuel_required(14), 2)

    def test_stage2(self):
        self.assertEqual(fuel_required_until_empty(fuel_required, 14), 2)
        self.assertEqual(fuel_required_until_empty(fuel_required, 1969), 966)

        self.assertEqual(fuel_required_until_empty(fuel_required_nofloor, 14), 2)
        self.assertEqual(fuel_required_until_empty(fuel_required_nofloor, 1969), 966)


if __name__ == "__main__":
    unittest.main()
