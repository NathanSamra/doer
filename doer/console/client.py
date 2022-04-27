from datetime import date, datetime
from typing import List

from doer import storage
from doer import config
from doer.model import Data, Priority


def _collect_items(date_) -> List[Priority]:
    items: List[Priority] = []
    line = input(f'List items for {date_}\n')

    while line:
        items.append(Priority(line))
        line = input('Anymore?\n')

    return items


def _order_items(items: List[Priority]) -> List[Priority]:
    result: List[Priority] = []
    remaining = items

    for priority in range(6):
        if len(remaining) == 0:
            return result

        print('Remaining:')
        for i, item in enumerate(remaining, start=1):
            print(f'{i}. {item.name}')
        choice = input('Select the top priority, or press enter to end.\n')
        if not choice:
            return result

        result.append(remaining.pop(int(choice) - 1))

    return result


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
        items: List[Priority] = []
        if len(day.priorities) > 0:
            self.show_priorities(date_)
            items.extend(day.priorities)

        items.extend(_collect_items(date_))
        day.priorities = _order_items(items)
        self.data.set_day(date_, day)
        self.show_priorities(date_)

    def copy_priorities(self, date_from: date, date_to: date):
        from_ = self.data.day(date_from)
        self.data.set_day(date_to, from_)
        self.show_priorities(date_to)

    def show(self, date_: date):
        self.show_priorities(date_)
        self.show_log(date_)

    def show_priorities(self, date_: date):
        day = self.data.day(date_)
        focus = day.focus.name if day.focus is not None else ''

        print(f'Priorities for {date_} are:')
        for i, priority in enumerate(day.priorities, start=1):
            line = f'{i}. {priority.name}'

            if priority.name == focus:
                line += '*'
                focus = ''

            if priority.done:
                line += ' - done'

            print(line)

        if focus != '':
            print(f'\nFocus: {focus}')

        print('\n')

    def show_log(self, date_: date):
        day = self.data.day(date_)
        print(f'Log for {date_} is:')

        for focus in day.log:
            start = focus.start.isoformat('minutes')
            print(f'{start} - {focus.name}')

            for break_ in focus.breaks:
                break_start = break_.start_time.isoformat('minutes')
                break_end = break_.end_time.isoformat('minutes')
                print(f'\t{break_start} - {break_end}')

        print('\n')

    def tick(self, date_: date, id_: int):
        self._set_tick(date_, id_, True)

    def un_tick(self, date_: date, id_: int):
        self._set_tick(date_, id_, False)

    def _set_tick(self, date_: date, id_: int, state: bool):
        day = self.data.day(date_)
        max_id = len(day.priorities) - 1
        if id_ > max_id:
            print(f'id {id_} invalid, maximum is {max_id}')
            return

        day.priorities[id_].done = state
        self.data.set_day(date_, day)

    def _today(self):
        return self.data.day(date.today())

    def _set_today(self, day):
        self.data.set_day(date.today(), day)

    def set_focus_to_priority(self, id_: int):
        day = self._today()
        max_id = len(day.priorities) - 1
        if id_ > max_id:
            print(f'id {id_} invalid, maximum is {max_id}')
            return

        day.focus = day.priorities[id_].name
        self._set_today(day)

    def set_focus(self, name: str):
        day = self._today()
        day.focus = name
        self._set_today(day)

    def start_break(self):
        day = self._today()
        day.focus.start_break()
        self._set_today(day)

    def end_break(self):
        day = self._today()
        day.focus.end_break()
        self._set_today(day)

    def end_day(self):
        day = self._today()
        day.end()
        self._set_today(day)
