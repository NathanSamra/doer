from datetime import date

from do import storage
from do import config
from do.model import Data


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
