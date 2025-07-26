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

    def as_dict(self):
        return asdict(self, dict_factory=custom_asdict_factory)

    @classmethod
    def keys(cls):
        return ['location', 'date', 'day_type']

    @classmethod
    def from_dict(cls, data:dict):
        data_copy = data.copy()
        if 'day_type' in data_copy and not isinstance(data_copy['day_type'], DayType):
            data_copy['day_type'] = DayType(int(data_copy['day_type']))

        return DateLocLog(**data)
    

def custom_asdict_factory(data):

    def convert_value(obj):
        if isinstance(obj, Enum):
            return obj.value
        return obj

    return dict((k, convert_value(v)) for k, v in data)