import datetime
import typing
from datetime import datetime, time


class Priority:
    def __init__(self, name):
        self.name: str = name
        self.done: bool = False

    def __eq__(self, other):
        return (self.name, self.done) == (other.name, other.done)


class Focus:
    class Break:
        def __init__(self, start_time: time):
            self.start_time: time = start_time
            self.end_time: time = time.max

        def end(self):
            self.end_time = datetime.now().time()

    def __init__(self, name: str, start: time):
        self.name: str = name
        self.start: time = start
        self.breaks: typing.List[Focus.Break] = []

    def start_break(self):
        new_break = self.Break(datetime.now().time())

        if len(self.breaks) > 0:
            last_break = self.breaks[-1]
            assert (last_break.end_time < new_break.start_time)

        self.breaks.append(new_break)

    def end_break(self):
        assert(len(self.breaks) > 0)
        break_ = self.breaks[-1]
        break_.end()


class Day:
    def __init__(self):
        self.priorities: typing.List[Priority] = []
        self.log: typing.List[Focus] = []
        self.end_time: typing.Optional[time] = None
        self.notes: typing.List[str] = []

    @property
    def focus(self) -> typing.Optional[Focus]:
        if len(self.log) == 0:
            return None

        return self.log[-1]

    @focus.setter
    def focus(self, name: str):
        if len(self.log) > 0 and self.log[-1].name == name:
            return

        self.log.append(Focus(name, datetime.now().time()))

    def end(self):
        self.end_time = datetime.now().time()
