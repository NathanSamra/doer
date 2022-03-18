import json
from datetime import date
from pathlib import Path

from appdirs import AppDirs

from do.metadata import __version__
from do.model.day import Day
from do.model.year import Year, year_from_json, year_to_json


class Data:
    def __init__(self, context: str = 'default'):
        dirs = AppDirs('do')
        self._database = Path(dirs.user_data_dir, context)
        if not self._database.exists():
            self._database.mkdir(parents=True)

    def year(self, year_num: int, ) -> Year:
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

    def set_year(self, year_num: int, year: Year):
        year_file = self._year_file(year_num)
        year_dict = year_to_json(year)
        year_dict['version'] = __version__

        with year_file.open('w+') as f:
            json.dump(year_dict, f, indent=4)

    def set_day(self, date_: date, day: Day):
        year = self.year(date_.year)
        year[date_] = day
        self.set_year(date_.year, year)

    def _year_file(self, year_num: int) -> Path:
        return self._database / f'{year_num}.json'
