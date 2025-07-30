import rich
import rich.prompt
from enum import Enum
from dataclasses import dataclass

class ActionResponseType(Enum):
    OK = 0
    CANCEL = 1

@dataclass
class Action:
    action: callable
    """callable object, the action to perform"""

    name: str
    """display name"""

    key: str
    """hotkey to press to select"""
    
@dataclass
class ActionResponse:
    response_type : ActionResponseType
    response_object: object



def prompt_action(prompt: str, actions:list[Action])->ActionResponse:
    str_items = [i.name for i in actions]
    assert len(str_items) == len(set([x.key for x in actions])), "Cannot have duplicate keys"

    str_items = []
    for idx,action in enumerate(actions):
        str_items.append(f"{action.name}")
    str_items_joined = "\n".join(str_items)
    msg = f"""{prompt}\n{str_items_joined}\nChoose any: """
    res = rich.prompt.Prompt.ask(msg, choices=[a.key for a in actions],show_choices=True)

    res_action = [x for x in actions if res == x.key]
    if res_action:
        return res_action[0]
    return None
        
    

def prompt_index(prompt:str, items=list())->str:
    assert items
    str_items = [str(i) for i in items]
    for idx in range(len(str_items)):
        str_items[idx] = f" {idx}). {str_items[idx]}"

    str_items_joined = "\n".join(str_items)
    
    msg = f"""{prompt}\n{str_items_joined}\nChoose any: """
    nums = [str(i) for i in range(len(items))]
    res = rich.prompt.IntPrompt.ask(msg, choices=nums,show_choices=True)

    
    if res is not None:
        return items[int(res)]
    return None


# thank you: https://code.activestate.com/recipes/134892/
class _Getch:
    """Gets a single character from standard input.  Does not echo to the
screen."""
    def __init__(self):
        try:
            self.impl = _GetchWindows()
        except ImportError:
            self.impl = _GetchUnix()

    def __call__(self): return self.impl()


class _GetchUnix:
    def __init__(self):
        import tty, sys

    def __call__(self):
        import sys, tty, termios
        fd = sys.stdin.fileno()
        old_settings = termios.tcgetattr(fd)
        try:
            tty.setraw(sys.stdin.fileno())
            ch = sys.stdin.read(1)
        finally:
            termios.tcsetattr(fd, termios.TCSADRAIN, old_settings)
        return ch


class _GetchWindows:
    def __init__(self):
        import msvcrt

    def __call__(self):
        import msvcrt
        return msvcrt.getch()