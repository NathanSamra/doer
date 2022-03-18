import sys


def _unrecognised(command):
    raise RuntimeError(f'Unrecognised command {command}')


def enter():
    command = sys.argv[1]
    if command == "plan":
        if len(sys.argv) >= 3:
            day = sys.argv[2]
            if day == "today":
                plan_today()
            elif day == "tomorrow":
                plan_tomorrow()
            else:
                _unrecognised(day)
        else:
            plan_tomorrow()
    else:
        _unrecognised(command)

