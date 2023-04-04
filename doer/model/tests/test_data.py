import random
import unittest
from datetime import date

from doer import storage
from doer.model.data import Data
from doer.model.day import Day, Priority
from doer.model.day_editor import DayEditor


class DataTests(unittest.TestCase):
    def test_write_and_read(self):
        priorities = [Priority(str(random.randint(0, 100))) for i in range(0, 6)]
        data = Data(storage.database(), "test")

        with DayEditor(date.today(), data) as day:
            day.priorities = priorities

        day2 = data.day(date.today())
        self.assertListEqual(priorities, day2.priorities)
