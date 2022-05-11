from datetime import date

from . import Data


class DayEditor:
    def __init__(self, date_: date, data: Data):
        self._data = data
        self._date = date_
        self._day = None

    def __enter__(self):
        self._day = self._data.day(self._date)
        return self._day

    def __exit__(self, exc_type, exc_val, exc_tb):
        self._data.set_day(self._date, self._day)
        self._day = None
