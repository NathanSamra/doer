import argparse
from datetime import date, timedelta

from do.console.client import Client


def _tomorrow():
    return date.today() + timedelta(days=1)


def enter():
    client = Client()

    parser = argparse.ArgumentParser()
    parser.add_argument('action', type=str, help='Action to perform, plan or show')

    day_group = parser.add_mutually_exclusive_group(required=False)
    day_group.add_argument('--today', action='store_true', help='Today')
    day_group.add_argument('--tomorrow', action='store_true', help='Tomorrow')
    day_group.add_argument('--day', type=str, help='Day in ISO format')

    args = parser.parse_args()
    date_ = _tomorrow()
    if args.today:
        date_ = date.today()
    elif args.day is not None:
        date_ = date.fromisoformat(args.day)

    if args.action == 'plan':
        client.plan_priorities(date_)
    elif args.action == 'show':
        client.show(date_)
    else:
        print(f'{args.action} is not a recognised command')
