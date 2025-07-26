from dataclasses import dataclass, asdict
from datetime import date
from enum import Enum

class DayType(Enum):
    FREE = 0
    WORK = 1


@dataclass
class DateLocLog:
    location: str
    date: date
    day_type: DayType
    
