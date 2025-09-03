import typer
from datetime import datetime
import locwork.locations
import locwork.logs
import locwork.interactive

app = typer.Typer(add_completion=False, invoke_without_command=True)

app.add_typer(locwork.locations.app, name="location")
app.add_typer(locwork.logs.app, name="log")

def interactive_log_date(day):
    from locwork.locations import _get_locations
    from locwork.logs import add as add_log
    locations = _get_locations()
    # locations.insert(0, "home")
    res = locwork.interactive.prompt_index("Select your work location", locations)
    if res is not None:
        add_log(res, day=day)

def interactive_log_today():
    interactive_log_date(datetime.today())
    
def interactive_quit():
    import sys
    sys.exit()


@app.command(short_help="interactive mode", hidden=True)
def interactive():
    from locwork.statpage import render_statpage_today   
    from locwork.statpage import PagedStatsViewer
    from locwork.logs import get_records
    records = get_records()
    # viewer = PagedStatsViewer(records)
    show_viewer = lambda x : PagedStatsViewer(records).show

    from locwork.interactive import Action
    
    actions = [
        Action(interactive_log_today, " > log today", 'l'),
        Action(show_viewer, " > stats", 's'),
        Action(interactive_quit, " > quit" , 'q')
    ]
    while(True):
        res = locwork.interactive.prompt_action("Choose an action", actions)
        if res:
            res.action()


@app.callback()
def main(ctx: typer.Context,
            interactive_mode: bool = typer.Option(False, "-i", "--interactive", help="runs interactive mode")):
    """
    Locwork is a simple tool that helps you track your work locations if you work in multiple offices,
    and have to (for some reason) track these locations on a daily basis.
    See the help sections for more information. Use the CLI for automation through scripts or specific 
    commands, or the interactive mode for user convenience.

    When going into interactive mode, it's important that you do not pass in any other commands, otherwise the -i flag is ignored.
    """
    if ctx.invoked_subcommand is None:
        if interactive_mode:
            ctx.invoke(interactive)
        else:
            print(ctx.get_help())
            raise typer.Exit()
if __name__ == "__main__":
    app()