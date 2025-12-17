from models import DateLocLog,DayType
from datetime import date, timedelta
from rich import print
from calendar import render_calendar
from rich.panel import Panel
from rich.align import Align
from rich.theme import Theme
from rich.console import Console
from rich import prompt
console = Console()

dates = [
    DateLocLog("home" , date(2025,6,30), DayType.FREE),
    DateLocLog("home" , date(2025,7,1), DayType.WORK),
    DateLocLog("work" , date(2025,7,2), DayType.WORK),
    DateLocLog("home" , date(2025,7,3), DayType.WORK),
    DateLocLog("home" , date(2025,7,10), DayType.WORK),
    DateLocLog("work" , date(2025,8,1), DayType.WORK),
]

cal = render_calendar(2025, 7, dates)
cal_text = '\n'.join(cal)

renderable = align(panel(cal_text), align="center")

def main():

    prompt.Prompt.ask("Choose a location:", choices=["1. home", "2. work"])
    console.print(renderable)
    pass
    
if __name__ == "__main__":
    main()


