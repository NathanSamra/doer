import argparse
import sys
from datetime import date, timedelta

from doer import metadata
from doer.console.client import Client


def _date_from_arg(arg) -> date:
    today = date.today()

    aliases = {
        "today": today,
        "tomorrow": date.today() + timedelta(days=1),
    }

    if arg in aliases:
        return aliases[arg]

    return date.fromisoformat(arg)


def _add_plan_action(parsers):
    def action(args):
        client = Client()
        client.plan_priorities(_date_from_arg(args.date_))

    plan_parser = parsers.add_parser('plan')
    plan_parser.add_argument('date_', type=str, help='Date to plan')
    plan_parser.set_defaults(func=action)


def _add_copy_action(parsers):
    def action(args):
        client = Client()
        client.copy_priorities(_date_from_arg(args.from_), _date_from_arg(args.to))

    copy_parser = parsers.add_parser('copy')
    copy_parser.add_argument('from_', type=str, help='Date to copy from. Can be \'today\', \'tomorrow\', or a date in '
                                                     'ISO format')
    copy_parser.add_argument('to', type=str, help='Date to copy from. Can be \'today\', \'tomorrow\', or a date in '
                                                  'ISO format')
    copy_parser.set_defaults(func=action)


def _add_show_action(parsers):
    def action(args):
        client = Client()
        client.show(_date_from_arg(args.date_))

    show_parser = parsers.add_parser('show')
    show_parser.add_argument('date_', type=str, help='Date to show')
    show_parser.set_defaults(func=action)


def _add_tick_action(parsers):
    def action(args):
        client = Client()
        client.tick(int(args.id_) - 1)

    tick_parser = parsers.add_parser('tick')
    tick_parser.add_argument('id_')
    tick_parser.set_defaults(func=action)


def _add_un_tick_action(parsers):
    def action(args):
        client = Client()
        client.un_tick(int(args.id_) - 1)

    un_tick_parser = parsers.add_parser('un-tick')
    un_tick_parser.add_argument('id_')
    un_tick_parser.set_defaults(func=action)


def _add_context_action(parsers):
    def action():
        client = Client()
        client.context()

    context_parser = parsers.add_parser('context')
    context_parser.set_defaults(func=action)


def _add_contexts_action(parsers):
    def action():
        client = Client()
        client.contexts()

    contexts_parser = parsers.add_parser('contexts')
    contexts_parser.set_defaults(func=action)


def _add_set_context_action(parsers):
    def action(args):
        client = Client()
        client.set_context(args.context)

    set_context_parser = parsers.add_parser('set_context')
    set_context_parser.add_argument('context', help='Context to switch to')
    set_context_parser.set_defaults(func=action)


def _add_set_focus(parsers):
    def action(args):
        client = Client()
        focus = args.focus
        if focus.isnumeric():
            client.set_focus_to_priority(int(focus) - 1)
        else:
            client.set_focus(focus)

    set_focus_parser = parsers.add_parser('set_focus')
    set_focus_parser.add_argument('focus', help='name or ID of focus')
    set_focus_parser.set_defaults(func=action)


def _add_start_break(parsers):
    def action():
        Client().start_break()

    start_break_parser = parsers.add_parser('start_break')
    start_break_parser.set_defaults(func=action)


def _add_end_break(parsers):
    def action():
        Client().end_break()

    end_break_parser = parsers.add_parser('end_break')
    end_break_parser.set_defaults(func=action)


def _add_end_day(parsers):
    def action():
        Client().end_day()

    end_day_parser = parsers.add_parser('end_day')
    end_day_parser.set_defaults(func=action)


def enter():
    parser = argparse.ArgumentParser()
    parser.add_argument('--version', action='version', version=metadata.version)

    action_parsers = parser.add_subparsers()
    _add_plan_action(action_parsers)
    _add_copy_action(action_parsers)
    _add_show_action(action_parsers)
    _add_tick_action(action_parsers)
    _add_un_tick_action(action_parsers)
    _add_context_action(action_parsers)
    _add_contexts_action(action_parsers)
    _add_set_context_action(action_parsers)
    _add_set_focus(action_parsers)
    _add_start_break(action_parsers)
    _add_end_break(action_parsers)
    _add_end_day(action_parsers)

    if len(sys.argv) == 1:
        sys.argv.append('--help')

    args = parser.parse_args()
    args.func(args)
