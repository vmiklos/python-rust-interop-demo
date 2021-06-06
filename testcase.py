#!/usr/bin/env python3

import unittest
import unittest.mock

import string_sum

def wrap(a, b):
    return string_sum.sum_as_string(a, b)


class TestStringSum(unittest.TestCase):
    def test_happy(self):
        self.assertEqual(string_sum.sum_as_string(1, 2), "3")

    def test_wrap(self):
        self.assertEqual(wrap(1, 2), "3")

    def test_mock(self):
        """Shows that even if a function is implemented in Rust, we can mock it in Python."""
        def mock_sum_as_string(a, b):
            return str(a + b + 1)
        with unittest.mock.patch("string_sum.sum_as_string", mock_sum_as_string):
            self.assertEqual(wrap(1, 2), "4")

if __name__ == '__main__':
    unittest.main()
