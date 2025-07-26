import os
import typer
from platformdirs import user_data_dir
from pathlib import Path

app = typer.Typer(invoke_without_command=True)

_location_store = Path(user_data_dir("locwork", ensure_exists=True)).joinpath("locations")
_location_store.touch(exist_ok=True)


def _get_locations():
    with open(_location_store, "r") as f:
        lines = f.readlines()
        lines = [l.strip() for l in lines if l.strip()]
        return lines

def _write_locations(locations):
    with open(_location_store, "w") as f:

        locations = "\n".join(locations)

        f.write(locations)



@app.command(short_help="add a new location",
             help="Adds a new location to the list of known locations.")
def add(name:str):
    locs = _get_locations()
    if name in locs:
        typer.echo(f"-- location '{name} already exists")
        return

    locs.append(name)
    _write_locations(locs)
    typer.echo(f"-- location '{name}' added --")
    
    
@app.command(short_help="remove a location",
             help="Removes a location from known locations. Does not update locwork db")
def remove(name:str):
    locs = _get_locations()
    if name not in locs:
        typer.echo(f"-- Unknown location '{name}' --")
        return

    locs.remove(name)
    _write_locations(locs)
    typer.echo(f"-- location '{name}' deleted --")



    # locs.append(name)
    # _write_locations(locs)
    # typer.echo(f"-- location '{name}' added --")



@app.command(name="list", short_help="list locations")
def list_locations():
    s = "\n".join(_get_locations())
    if not s:
        typer.echo("-- no locations added --")
    print(s)
    # typer.echo(s)

@app.callback()
def main(ctx: typer.Context):
    """Manage Locations"""
    if ctx.invoked_subcommand is None:
        # No subcommand was invoked, show help
        print(ctx.get_help())
        raise typer.Exit()