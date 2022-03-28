import typing
from datetime import date

from doer.model.day import Day

Year = typing.Dict[date, Day]


def year_to_json(year: Year):
    def day_to_json(day: Day):
        return {'priorities': day.priorities}

    return {'days': {date_.isoformat(): day_to_json(day) for date_, day in year.items()}}


def year_from_json(year_dict):
    year = {}
    if 'days' not in year_dict:
        return year

    for date_str, day_dict in year_dict['days'].items():
        date_ = date.fromisoformat(date_str)
        day = Day()
        if 'priorities' in day_dict:
            day.priorities = day_dict['priorities']

        year[date_] = day

    return year
