import calendar
import time
import unittest

import today

class TestTimer:
    def time(self):
        return float(calendar.timegm(time.strptime("2021-06-11", "%Y-%m-%d")))

class TestToday(unittest.TestCase):
    def test_happy(self):
        self.assertEqual(today.string_today(TestTimer()), "2021-06-11")

if __name__ == '__main__':
    unittest.main()
