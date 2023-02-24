import click
from util.repo import download_theme, list_themes

@click.group()
def cli():
    ...

@cli.command(help='Install a theme')
@click.argument('theme')
@click.option('--local', '-l', help='provide the path of local theme zip', default=False, is_flag=True)
def install(theme:str,local:bool=False):
    if local:
        click.echo('Installing from local')
    else:
        click.echo('Installing from remote')
    download_theme(theme)

@cli.command(help='Uninstall a theme')
@click.argument('theme')
def uninstall(theme:str):
    click.echo(f'Hello World! {theme}')

@cli.command(name='list',help='List all themes')
@click.option('--installed', '-i', help='list only installed themes', default=False, is_flag=True)
def _list(installed:bool=False):
    for theme in list_themes():
        click.echo(theme.get('theme').get('name'))

@cli.command(help='apply a theme')
@click.argument('theme')
def apply(theme:str):
    click.echo(f'Hello World! {theme}')