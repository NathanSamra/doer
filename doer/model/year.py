import typing
from datetime import date

import semantic_version
from semantic_version import Version, SimpleSpec

from doer.model.day import Day, Priority

Year = typing.Dict[date, Day]


def year_to_json(year: Year):
    def day_to_json(day: Day):
        return {'priorities': [{'name': priority.name, 'done': priority.done} for priority in day.priorities]}

    return {'days': {date_.isoformat(): day_to_json(day) for date_, day in year.items()}}


def year_from_json(year_dict):
    def priority_from_json(priority_dict: typing.Dict):
        priority = Priority(priority_dict['name'])
        priority.done = priority_dict['done']
        return priority

    json_version = None
    if 'version' in year_dict:
        json_version = Version(year_dict['version'])

    year = {}
    if 'days' not in year_dict:
        return year

    for date_str, day_dict in year_dict['days'].items():
        date_ = date.fromisoformat(date_str)
        day = Day()
        if 'priorities' in day_dict:
            priorities_dicts = day_dict['priorities']
            if SimpleSpec('<1.2.0').match(json_version):
                day.priorities = [Priority(name) for name in priorities_dicts]
            else:
                day.priorities = [priority_from_json(priority_dict) for priority_dict in priorities_dicts]

        year[date_] = day

    return year
