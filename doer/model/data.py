import json
import os
import typing
from datetime import date
from pathlib import Path

from doer.metadata import __version__
from doer.model.day import Day
from doer.model.year import Year, year_from_json, year_to_json


class Data:
    def __init__(self, database: Path, context: str):
        self._root = database
        self._context = context
        if not self._database.exists():
            self._database.mkdir(parents=True)

    @property
    def _database(self):
        return self._root / self._context

    def year(
        self,
        year_num: int,
    ) -> Year:
        year_file = self._year_file(year_num)
        if not year_file.exists():
            return {}

        with year_file.open('r') as f:
            return year_from_json(json.load(f))

    def day(self, date_: date) -> Day:
        year = self.year(date_.year)
        if date_ in year:
            return year[date_]

        return Day()

    @property
    def last_date(self) -> date:
        year = self.year(self.last_year)
        today = date.today()

        for date_ in sorted(year.keys(), reverse=True):
            if date_ <= today:
                return date_

        raise RuntimeError("No dates saved in year")

    @property
    def last_year(self) -> int:
        current_year = date.today().year

        for year_num in [int(year_file.stem) for year_file in sorted(self._database.glob("*.json"), reverse=True)]:
            if year_num <= current_year:
                return year_num

        raise RuntimeError("No year data found")

    def set_year(self, year_num: int, year: Year):
        year_file = self._year_file(year_num)
        year_dict = year_to_json(year)
        year_dict['version'] = str(__version__)

        with year_file.open('w+') as f:
            json.dump(year_dict, f, indent=4)

    def set_day(self, date_: date, day: Day):
        year = self.year(date_.year)
        year[date_] = day
        self.set_year(date_.year, year)

    def _year_file(self, year_num: int) -> Path:
        return self._database / f'{year_num}.json'
