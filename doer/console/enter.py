import argparse
from datetime import date, timedelta

from doer.console.client import Client


def _date(args) -> date:
    date_ = date.today() + timedelta(days=1)
    if args.today:
        date_ = date.today()
    elif args.day is not None:
        date_ = date.fromisoformat(args.day)

    return date_


def _plan(args):
    client = Client()
    client.plan_priorities(_date(args))


def _show(args):
    client = Client()
    client.show(_date(args))


def _context(args):
    client = Client()
    client.context()


def _contexts(args):
    client = Client()
    client.contexts()


def _set_context(args):
    client = Client()
    client.set_context(args.context)


def _add_date_group(parser):
    group = parser.add_mutually_exclusive_group(required=False)
    group.add_argument('--today', action='store_true', help='Today')
    group.add_argument('--tomorrow', action='store_true', help='Tomorrow')
    group.add_argument('--day', type=str, help='Day in ISO format')


def enter():
    parser = argparse.ArgumentParser()
    action_parsers = parser.add_subparsers()

    plan_parser = action_parsers.add_parser('plan')
    _add_date_group(plan_parser)
    plan_parser.set_defaults(func=_plan)

    show_parser = action_parsers.add_parser('show')
    _add_date_group(show_parser)
    show_parser.set_defaults(func=_show)

    context_parser = action_parsers.add_parser('context')
    context_parser.set_defaults(func=_context)

    contexts_parser = action_parsers.add_parser('contexts')
    contexts_parser.set_defaults(func=_contexts)

    set_context_parser = action_parsers.add_parser('set_context')
    set_context_parser.add_argument('context', help='Context to switch to')
    set_context_parser.set_defaults(func=_set_context)

    args = parser.parse_args()
    args.func(args)
