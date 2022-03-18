from datetime import date, timedelta

from do.model import Data


def _tomorrow() -> date:
    return date.today() + timedelta(days=1)


class Do:
    def __init__(self, context: str = None):
        if context:
            self.data = Data(context)
        else:
            self.data = Data()

    def plan_priorities(self, date_: date = _tomorrow()):
        day = self.data.day(date_)
        print(f'List all your items for {date_}')

        items = []
        line = input(f'List all your items for {date_}')

        while line:
            items.append(line)
            line = input('Anymore?')

        print('your items:')
        for i, item in enumerate(items, start=1):
            print(f'{i}. {item}')

        choice = input('Please order your items')
        success = False
        nums = []
        while not success:
            nums = [int(num) for num in choice.split()]
            success = True

        day.priorities = [items[i - 1] for i in nums]

        print('done')
        self.data.set_day(date_, day)

    def show(self, date_: date):
        day = self.data.day(date_)
        print(f'Priorities for {date_} are:')
        for i, priority in enumerate(day.priorities, start=1):
            print(f'{i}. {priority}')

    def show_today(self):
        self.show(date.today())

    def show_tomorrow(self):
        self.show(_tomorrow())
