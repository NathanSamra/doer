from datetime import date
from typing import List

from do import storage
from do import config
from do.model import Data, Day


def _collect_items(date_) -> List[str]:
    items = []
    line = input(f'List items for {date_}\n')

    while line:
        items.append(line)
        line = input('Anymore?\n')

    return items


def _order_items(items) -> List[str]:
    result = []
    remaining = items

    for priority in range(6):
        if len(remaining) == 0:
            return result

        print('Remaining:')
        for i, item in enumerate(remaining, start=1):
            print(f'{i}. {item}')
        choice = input('Select the top priority, or press enter to end.\n')
        if not choice:
            return result

        result.append(remaining.pop(int(choice) - 1))

    return result


def _show_day(day: Day, date_: date):
    print(f'Priorities for {date_} are:')
    for i, priority in enumerate(day.priorities, start=1):
        print(f'{i}. {priority}')


class Client:
    def __init__(self):
        self.data = Data(storage.database(), config.context())

    @staticmethod
    def set_context(new_context):
        config.set_context(new_context)

    @staticmethod
    def contexts():
        for context_ in config.contexts():
            print(context_)

    @staticmethod
    def context():
        print(config.context())

    def plan_priorities(self, date_: date):
        day = self.data.day(date_)
        items = []
        if len(day.priorities) > 0:
            _show_day(day, date_)
            items.extend(day.priorities)

        items.extend(_collect_items(date_))
        day.priorities = _order_items(items)
        self.data.set_day(date_, day)
        _show_day(day, date_)

    def show(self, date_: date):
        day = self.data.day(date_)
        _show_day(day, date_)
