import os
import typer
from typing_extensions import Annotated
import csv
from datetime import date, datetime
from platformdirs import user_data_dir
from pathlib import Path
from locwork.models import DateLocLog, DayType
from locwork.locations import _get_locations
app = typer.Typer(invoke_without_command=True)


_log_store = Path(user_data_dir("locwork", ensure_exists=True)).joinpath("records.csv")

def _write_records(records:dict[str, DateLocLog]):

    if not records:
        # clear records file
        with open(_log_store, "w" , encoding="utf-8"):
            return

    first = records[list(records.keys())[0]]

    with open(_log_store, "w", encoding="utf-8") as f:
        fields = DateLocLog.keys()

        writer = csv.DictWriter(f, fieldnames=fields)
        writer.writeheader()
        for day, record in records.items():
            writer.writerow(record.as_dict())
            
def _get_records():

    if not os.path.exists(_log_store):
        return {}

    records = {}
    try:
        with open(_log_store, "r", encoding="utf-8") as f:
            fields = DateLocLog.keys()

            reader = csv.DictReader(f, fieldnames=fields)
            next(reader) # skip headewrs
            for row in reader:
                record = DateLocLog.from_dict(row)
                records[record.date] = record

    except Exception as e:
        print(e)
        return {}

    return records



@app.command(help="Register a new record for a specified date (or today). Only one record is kept per day. Adding another record on an existing date overwrites it.",
             short_help="Add/Update a record for a date. (must be paired with -t or -d)")
def add(location:str,
    today: Annotated[bool, typer.Option("--today", "-t", help="Add a record for today", )] = False,
    day: Annotated[datetime, typer.Option("--date", "-d", help="Add a record with a specifc date", )] = None,
    holiday: Annotated[bool, typer.Option('--holiday', help="Is this day a holiday?")] = False
    ):

    if not today and not day:
        typer.echo("-- either --today (-t) or --date (-d) are required --")
        return
    if today and day:
        typer.echo("-- today and date are mutually exclusive --")
        return
    if day:
        assert isinstance(day, datetime), f"expected datetime, got {type(day)}"

    day_type = DayType.WORK
    if holiday:
        day_type = DayType.FREE

    if today:
        day = date.today()
    else:
        day = date(day.year, day.month, day.day)

    if location not in _get_locations():
        typer.echo("-- location is not recognized --")
        return
    
    record = DateLocLog(location, day, day_type)
    records = _get_records()
    updated = False

    if record.date.isoformat() in records.keys():
        updated = True
    records[record.date.isoformat()] = record
    _write_records(records)
    if updated:
        typer.echo(f"-- updated record: {record.date} , {record.location} --")
    else:
        typer.echo(f"-- added record: {record.date} , {record.location} --")

@app.command(name="list",
help="Lists all records (for now), will allow you to filter stored records.",
short_help="Lists records based on filter.",
hidden=True)
def list_records():
    records = _get_records()
    # typer.echo(records)
    typer.echo("-- Not implemented yet --")




@app.callback()
def main(ctx: typer.Context):
    """Manage locwork records"""
    if ctx.invoked_subcommand is None:
        # No subcommand was invoked, show help
        print(ctx.get_help())
        raise typer.Exit()