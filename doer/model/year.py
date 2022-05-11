import typing
from datetime import date, datetime, time

import semantic_version
from semantic_version import Version, SimpleSpec

from doer.model.day import Day, Priority, Focus

Year = typing.Dict[date, Day]


def year_to_json(year: Year):
    def day_to_json(day: Day):
        day_dict = {
            'priorities': [{'name': priority.name, 'done': priority.done} for priority in day.priorities],
            'log': [{
                'name': focus.name,
                'start': focus.start.isoformat(),
                'breaks': [{
                    'start': break_.start_time.isoformat(),
                    'end': break_.end_time.isoformat()
                } for break_ in focus.breaks]
            } for focus in day.log],
            'notes': day.notes
        }

        if day.end_time is not None:
            day_dict['end_time'] = day.end_time.isoformat()

        return day_dict

    return {'days': {date_.isoformat(): day_to_json(day) for date_, day in year.items()}}


def year_from_json(year_dict):
    def priority_from_json(priority_dict: typing.Dict):
        priority = Priority(priority_dict['name'])
        priority.done = priority_dict['done']
        return priority

    def focus_from_json(focus_dict: typing.Dict):
        focus = Focus(focus_dict['name'], time.fromisoformat(focus_dict['start']))

        for break_dict in focus_dict['breaks']:
            break_ = Focus.Break(time.fromisoformat(break_dict['start']))
            break_.end_time = time.fromisoformat(break_dict['end'])
            focus.breaks.append(break_)

        return focus

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

        if 'log' in day_dict:
            day.log = [focus_from_json(focus_dict) for focus_dict in day_dict['log']]

        if 'end_time' in day_dict:
            day.end_time = time.fromisoformat(day_dict['end_time'])

        if 'notes' in day_dict:
            day.notes = day_dict['notes']

        year[date_] = day

    return year
