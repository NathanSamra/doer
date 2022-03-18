import random
import unittest
from datetime import date

from do import storage
from do.model.data import Data
from do.model.day import Day


class DataTests(unittest.TestCase):
    def test_write_and_read(self):
        priorities = [str(random.randint(0, 100)) for i in range(0, 6)]
        data = Data(storage.database(), 'test')

        day = Day()
        day.priorities = priorities
        data.set_day(date.today(), day)

        day2 = data.day(date.today())
        self.assertListEqual(priorities, day2.priorities)
