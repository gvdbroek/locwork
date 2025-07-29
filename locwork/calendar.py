from locwork.models import DateLocLog,DayType
from datetime import date, timedelta
from rich.panel import Panel
from rich.align import Align

# todo: move this to a seperate file and theme it

OFFICE_LOCATIONS = [
    "work"
]
HOME_LOCATIONS = [
    "home"
]

STYLES = {
    None: "grey42",
    DayType.FREE: "",
    DayType.WORK: "underline",
    "HOME": "green underline",
    "OFFICE": "yellow underline",
    "HEADER": "grey93 bold",
    "LINES": "grey42"
}

def _style(text, color):
    return f'[{color}]{text}[/{color}]'



def render_weekday(month, date:date, log:DateLocLog|None):
    col = STYLES[None]

    if log:
        col = STYLES.get(log.day_type, "yellow underline")
        if log.location in HOME_LOCATIONS:
            col = STYLES.get("HOME", "green underline")
    
    if month != date.month:
        col = col + " italic"

    

    return _style(f'{date.day:02}', col)


def render_calendar(year:int, month:int, logs:[DateLocLog])->list[str]: 
    data = []
    headers = [" m", " t" , " w" , " t", " f", " s", " s"]
    # headers = [_style(h, COLORS['HEADER']) for h in headers]

    logs_dict = {d.date.isoformat():d for d in logs}
    
    r = " | ".join(headers)
    row_sep = len(r) * '-'

    r = _style(r, STYLES['HEADER'])
    row_sep = _style(row_sep, STYLES['LINES'])

    days = []
    for daynum in range(1,32):
        try:
            d = date(year, month, daynum)
            days.append(d)
        except ValueError:
            break
    
    first_day = days[0]
    padding = first_day.weekday()
    for p in range(padding):
        delta = timedelta(p+1)
        pad_day = date(year, month, 1) - delta
        days.insert(0, pad_day)

    rows = [r, row_sep]
    for i in range(0,len(days), 7):
        week_days = [d for d in days[i:i+7]]
        week_logs = [logs_dict.get(d.isoformat(), None) for d in week_days]
        row = [render_weekday(month,wd,log) for wd,log in zip(week_days, week_logs)]
        row = " | ".join(row)
        rows.append(row)
    
    return rows




