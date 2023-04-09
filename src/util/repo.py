import git
import toml
import os
import aiohttp
from rich import progress
import rich.theme
from rich.tree import Tree
from rich.console import Console
from git import RemoteProgress
from .objects import PartialTheme
from . import config


console = Console()

class Git:
    
    async def download_theme(theme:str|PartialTheme):
        if isinstance(theme,str):
            if theme.startswith('git+'):
                print('cloning from git')
                repo = theme[4:]
                if theme[-1] == '/':theme = theme[:-1]
                return git.Repo.clone_from(repo, os.path.join(config.THEMEPATH,theme.strip().split('/')[-1]) , progress=CloneProgress())
        
            for repo_theme in await Git.list_themes():
                if repo_theme.name == theme:
                    theme = repo_theme
                    break

        dist = os.path.join(config.THEMEPATH,theme.name)

        if os.path.exists(dist) and os.path.isdir(dist):
            console.print(f'[bold yellow]Theme {theme.name} already exists, updating theme[/]')
            repo = git.Repo(dist)
            repo.remotes.origin.pull(progress=CloneProgress())
        else:
            try:
                git.Repo.clone_from(theme._git, dist , progress=CloneProgress())
            except git.exc.GitCommandError as e:
                console.print(f'[bold red]Error cloning {theme.name}[/] : {e}')
        # TODO: do theme parsing and converting partial to full theme

                
    async def list_themes():
        l = []
        if os.path.exists(os.path.join(config.CACHEPATH,'theme_repo')):
            repo = git.Repo(os.path.join(config.CACHEPATH,'theme_repo'))
            repo.remotes.origin.pull(progress=CloneProgress())
        else:
            repo = git.Repo.clone_from(config.THEMEREPO, os.path.join(config.CACHEPATH,'theme_repo'), progress=CloneProgress())
        async with aiohttp.ClientSession() as session:
            for repo in progress.track(repo.iter_submodules(),total=len(repo.submodules),description='Parsing themes'):
                if partial:=await Git.get_partial(*repo.url.split('/')[-2:],session=session):
                    l.append(partial)
        return l

    async def get_partial(user,repo,branch:str='main',session=None):
        if not session:
            async with aiohttp.ClientSession() as session:
                return await Git.get_partial(user,repo,session=session)
        async with session.get(url:=f'https://raw.githubusercontent.com/{user}/{repo}/{branch}/theme.toml') as resp:
            if resp.status != 200:
                return await Git.get_partial(user,repo,branch='master',session=session)
            theme_toml = toml.loads(await resp.text())
            return await PartialTheme.from_toml(theme_toml,user,repo,branch)
    
    async def select_theme(partials=None)->PartialTheme:
        tree = Tree('[yellow bold]Themes')
        partials = partials or await Git.list_themes()
        for i,theme in enumerate(partials):
            t = tree.add(f'[green bold]{i+1}:{theme.name}[/][dim] ({theme.author})')
            if theme.subthemes:
                for subtheme in theme.subthemes:
                    t.add(subtheme.name)
        console.print(tree)
        inp = console.input('[bold blue]Select a theme to install[/] : ')
        
        if inp.isdigit():
            partial = partials[int(inp)-1]
        elif inp in [theme.name for theme in partials]:
            partial = [theme for theme in partials if theme.name == inp][0]
        else:
            console.print('[bold red]Invalid input[/]')
            return await Git.select_theme()
        return partial

class CloneProgress(RemoteProgress):
    OP_CODES = [
        "BEGIN",
        "CHECKING_OUT",
        "COMPRESSING",
        "COUNTING",
        "END",
        "FINDING_SOURCES",
        "RECEIVING",
        "RESOLVING",
        "WRITING",
    ]
    OP_CODE_MAP = {
        getattr(git.RemoteProgress, _op_code): _op_code for _op_code in OP_CODES
    }

    theme = rich.theme.Theme({
        "bar.complete": "bold blue",
        "bar.finished": "bold blue",
        "bar.back": "bold dim magenta",
    })

    def __init__(self):
        super().__init__()
        self.progressbar = progress.Progress(
            progress.SpinnerColumn(),
            progress.TextColumn("[progress.description]{task.description}"),
            progress.BarColumn(),
            progress.TextColumn("[progress.percentage]{task.percentage:>3.0f}%"),
            "eta",
            progress.TimeRemainingColumn(),
            progress.TextColumn("{task.fields[message]}"),
            console=Console(theme=self.theme),
            transient=False,
        )
        self.progressbar.start()
        self.active_task = None     
    
    def __del__(self) -> None:
        self.progressbar.stop()

    @classmethod
    def get_curr_op(cls, op_code: int) -> str:
        """Get OP name from OP code."""
        op_code_masked = op_code & cls.OP_MASK
        return cls.OP_CODE_MAP.get(op_code_masked, "?").title()


    def update(self, op_code, cur_count, max_count=None, message=''):
        if op_code & self.BEGIN:
            self.curr_op = self.get_curr_op(op_code)
            self.active_task = self.progressbar.add_task(
                description=self.curr_op,
                total=max_count,
                message=message,
            )

        self.progressbar.update(
            task_id=self.active_task,
            completed=cur_count,
            message=message,
        )

        if op_code & self.END:
            self.progressbar.update(
                task_id=self.active_task,
                message=f"[bright_black]{message}",
            )
    
    


