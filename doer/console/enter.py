import argparse
import sys
from datetime import date, timedelta

from doer import metadata
from doer.console.client import Client


def _tomorrow() -> date:
    return date.today() + timedelta(days=1)


def _date_from_args(args) -> date:
    date_ = _tomorrow()
    if args.today:
        date_ = date.today()
    elif args.day is not None:
        date_ = date.fromisoformat(args.day)

    return date_


def _date_from_str(date_str) -> date:
    if date_str == 'today':
        return date.today()

    if date_str == 'tomorrow':
        return _tomorrow()

    return date.fromisoformat(date_str)


def _plan(args):
    client = Client()
    client.plan_priorities(_date_from_args(args))


def _copy(args):
    client = Client()
    client.copy_priorities(_date_from_str(args.from_), _date_from_str(args.to))


def _show(args):
    client = Client()
    client.show(_date_from_args(args))


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
    parser.add_argument('--version', action='version', version=metadata.version)

    action_parsers = parser.add_subparsers()

    plan_parser = action_parsers.add_parser('plan')
    _add_date_group(plan_parser)
    plan_parser.set_defaults(func=_plan)

    copy_parser = action_parsers.add_parser('copy')
    copy_parser.add_argument('from_', type=str, help='Date to copy from. Can be \'today\', \'tomorrow\', or a date in '
                                                     'ISO format')
    copy_parser.add_argument('to', type=str, help='Date to copy from. Can be \'today\', \'tomorrow\', or a date in '
                                                  'ISO format')
    copy_parser.set_defaults(func=_copy)

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

    if len(sys.argv) == 1:
        sys.argv.append('--help')

    args = parser.parse_args()
    args.func(args)
