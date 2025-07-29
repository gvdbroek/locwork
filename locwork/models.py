from dataclasses import dataclass, asdict
from datetime import date
from enum import Enum
import __future__

class DayType(Enum):
    FREE = 0
    WORK = 1


    
@dataclass
class DateLocLog:
    location: str
    date: str
    day_type: DayType

    def as_dict(self):
        return asdict(self, dict_factory=custom_asdict_factory)

    @classmethod
    def keys(cls):
        return ['location', 'date', 'day_type']

    @classmethod
    def from_dict(cls, data:dict):
        data_copy = data.copy()
        data["date"] = date.fromisoformat(data["date"])

        
        if 'day_type' in data_copy and not isinstance(data_copy['day_type'], DayType):
            data_copy['day_type'] = DayType(int(data_copy['day_type']))

        return DateLocLog(**data)
    

def custom_asdict_factory(data):

    def convert_value(obj):
        if isinstance(obj, Enum):
            return obj.value
        return obj

    return dict((k, convert_value(v)) for k, v in data)


class DateLocMap:
    """
        A map/dict of dateloc locs
    """

    def __init__(self):
        self._map = {}

    def __getitem__(self, key:str, default=None):
        assert isinstance(key, str)
        return self._map.get(key, default)

    def __setitem__(self, key:str, item:DateLocLog):
        assert isinstance(key, str)
        assert isinstance(item, DateLocLog)
        self._map[key] = item
        
    def keys(self):
        return sorted(self._map.keys())

    def items(self):
        for k,v in self._map.items():
            yield k,v
    

