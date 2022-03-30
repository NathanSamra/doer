from datetime import date

from django.http import HttpResponse
from django.shortcuts import render

from doer import storage, config
from doer.model import Data


def today(_request):
    data = Data(storage.database(), config.context())
    today_ = data.day(date.today())
    today_str = ''
    for i, priority in enumerate(today_.priorities, 1):
        today_str += f'{i}. {priority}\n'

    return HttpResponse(f"Today's priorities:\n\n{today_str}", content_type='text/plain')
