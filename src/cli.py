import asyncclick as click
from util.repo import Git
from rich.tree import Tree
from rich.console import Console

@click.group()
async def cli():
    ...

console = Console()

@cli.command(help='Install a theme')
@click.argument('theme')
@click.option('--local', '-l', help='provide the path of local theme zip', default=False, is_flag=True)
async def install(theme:str,local:bool=False):
    if local:
        click.echo('Installing from local')
    else:
        click.echo('Installing from remote')
    await Git.download_theme(theme)

@cli.command(help='Uninstall a theme')
@click.argument('theme')
async def uninstall(theme:str):
    click.echo(f'Hello World! {theme}')

@cli.command(name='list',help='List all themes')
@click.option('--installed', '-i', help='list only installed themes', default=False, is_flag=True)
async def _list(installed:bool=False):
    tree = Tree('[yellow bold]Themes')
    for theme in await Git.list_themes():
        t = tree.add(f'[green bold]{theme.name}[/][dim] ({theme.author})')
        if theme.subthemes:
            for subtheme in theme.subthemes:
                t.add(subtheme.name)
    console.print(tree)

@cli.command(help='apply a theme')
@click.argument('theme')
async def apply(theme:str):
    click.echo(f'Hello World! {theme}')