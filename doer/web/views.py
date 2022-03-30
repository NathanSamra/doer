from datetime import date

from django.shortcuts import render

from doer import storage, config
from doer.model import Data


def today(request):
    data = Data(storage.database(), config.context())
    today_ = data.day(date.today())
    context = {'today': today_}
    return render(request, 'today.html', context)
