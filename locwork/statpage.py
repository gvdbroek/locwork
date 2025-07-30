import sys
from datetime import date, timedelta
from locwork.logs import get_records
from locwork.models import DateLocLog, DateLocMap
import locwork.calendar
from enum import Enum
import rich
from rich.panel import Padding, Panel
from rich.console import Group
from rich.text import Text
from rich.columns import Columns
from rich.table import Table, Column, Row
from rich.align import Align
from collections import defaultdict
from rich.markdown import Markdown
from rich.console import Console
from locwork.interactive import _Getch
console = Console()


def iter_dates(date_start, date_end) -> list[date]:
    assert isinstance(date_start, date)
    assert isinstance(date_end, date)
    assert date_start < date_end

    delta = date_end - date_start
    
    for i in range(delta.days):
        yield date_start + timedelta(days=i)

def filter_records_between_dates(records: DateLocMap, start_date:date, end_date: date)->DateLocMap:
    start_date = start_date.isoformat()
    end_date = end_date.isoformat()
    return {k:v for k,v in records.items() if start_date <= k < end_date}

def get_month_range_for_date(day:date)->(date,date):
    """Returns month range (start, end) as tuple"""
    begin = date(day.year, day.month, 1)
    next_month = begin + timedelta(days=32)
    end = date(next_month.year, next_month.month, 1)
    return begin, end


def _render_locations_block(records: DateLocMap, date_start:date, date_end:date):
    
    locations = set([v.location for k,v in records.items()])
    
    location_counts = defaultdict(int)

    for k,v in records.items():
        location_counts[v.location] += 1
        
    days_recorded = sum(location_counts.values())
    
    places = [Text(key) for key in location_counts.keys()]
    places = list(location_counts.keys())
    places_counts = location_counts.values()
    
    sorted_places = sorted(places, key=lambda x: location_counts[x], reverse=True)
    
    

    table = Table(show_header=False, show_lines=False, show_edge=False)
    
    table.add_column("Location")
    table.add_column("Logged")
    table.add_column("percentage")
    for loc in sorted_places:
        count = location_counts[loc]
        table.add_row(loc, str(count) , f"{count/days_recorded * 100:0.1f}%")

    range_weekdays = 0
    range_days = 0

    for day in iter_dates(date_start, date_end):
        if day.isoweekday() < 6:
            range_weekdays += 1
        range_days += 1
        
    
    texts = [
        Markdown("## Distribution", justify="left"),
        
        table,
        Markdown("## Workdays", justify="left"),
        Text("This month %s" % range_weekdays),
        Text("Logged  %s" % days_recorded),
        Text("Unlogged  %s" % (range_weekdays - days_recorded))
    ]

    
    return Group(*texts)
    

def _render_calendar_block(records, date_start:date, date_end:date):
    from locwork.calendar import render_calendar
    #
    # todo:
    # rewrite calender to take dict of values

    cal_text =  render_calendar(date_start.year, date_start.month, records.values())
    renderable = Align(
        Group(
            *cal_text), 
        align="center")
    return renderable

def render_statpage_today():

    
    records = get_records()
    today = date.today()
    month_start, month_end = get_month_range_for_date(today)
    render_statpage(month_start, month_end, records)

def render_statpage(date_start, date_end, records):
    
    month_start = date_start
    month_end = date_end

    
    filtereded_records = filter_records_between_dates(records,month_start, month_end)
    
    title = Markdown("## Days worked at")
    
    calendar_pane = rich.panel.Panel(_render_calendar_block(filtereded_records, month_start, month_end), title=month_start.isoformat())
    stat_block = rich.panel.Panel(_render_locations_block(filtereded_records, month_start, month_end))
    ui = rich.console.Group(

        calendar_pane,
        stat_block
    )
    console.print(ui)


def interactive_statpage():
    records = get_records()
    viewer = PagedStatsViewer(records)
    pass

class PagedStatsViewer:
    def __init__(self, locmap:DateLocMap):
        self._data = locmap

        self._start_date, self._end_date = get_month_range_for_date(date.today())

        self._actions = {
            'l': self._next_range,
            'h': self._previous_range,
            'q': lambda: sys.exit()
        }

    
    def show(self):
        with console.screen():
            self._wait_for_input()

    def _render_page(self):
        render_statpage(self._start_date, self._end_date, self._data)
        
    def _previous_range(self):
        prev_month = self._start_date - timedelta(days=1)
        self._start_date, self._end_date = get_month_range_for_date(prev_month)

    def _next_range(self):
        self._start_date, self._end_date = get_month_range_for_date(self._end_date)

    def _wait_for_input(self):
        while True:
            console.clear()
            self._render_page()
            c = _Getch()()
            action = self._actions.get(c, None)
            if action:
                action()
            


