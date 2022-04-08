import typing


class Priority:
    def __init__(self, name):
        self.name: str = name
        self.done: bool = False


class Day:
    def __init__(self):
        self.priorities: typing.List[Priority] = []
