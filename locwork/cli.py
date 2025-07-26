import typer
import locwork.locations
import locwork.logs
app = typer.Typer(add_completion=False, invoke_without_command=True)

app.add_typer(locwork.locations.app, name="location")
app.add_typer(locwork.logs.app, name="log")

@app.command(short_help="interactive mode")
def interactive():
    typer.echo("-- welcome.. but this is a todo --")

@app.callback()
def main(ctx: typer.Context,
            interactive_mode: bool = typer.Option(False, "-i", "--interactive", help="runs interactive mode")):
    """
    Locwork is a simple tool that helps you track your work locations if you work in multiple offices,
    and have to (for some reason) track these locations on a daily basis.
    See the help sections for more information. Use the CLI for automation through scripts or specific 
    commands, or the interactive mode for user convenience.
    """
    if ctx.invoked_subcommand is None:
        if interactive_mode:
            ctx.invoke(interactive)
        else:
            print(ctx.get_help())
            raise typer.Exit()
if __name__ == "__main__":
    app()