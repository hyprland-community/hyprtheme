import git
import toml
import os
import aiohttp
from rich import progress, console
import rich.theme
from git import RemoteProgress
from .objects import PartialTheme
from . import config

class Git:
    async def download_theme(theme:str):
        if theme.startswith('git+'):
            repo = theme[4:]
            return git.Repo.clone_from(repo, os.path.join(config.THEMEPATH,theme.split('/')[-1]) , progress=CloneProgress())
        
        for repo_theme in await Git.list_themes():
            if repo_theme.name == theme:
                repo = repo_theme._repo

                # TODO: add support for branches
                if (r:=repo.split('/'))[-2] == 'tree':
                    branch = r[-1]
                    repo = '/'.join(r[:-2])
                
                git.Repo.clone_from(repo, os.path.join(config.THEMEPATH,theme) , progress=CloneProgress(),branch = branch)

                # TODO: do theme parsing and converting partial to full theme
                return
                
    async def list_themes():
        l = []
        if os.path.exists(os.path.join(config.CACHEPATH,'theme_repo')):
            repo = git.Repo(os.path.join(config.CACHEPATH,'theme_repo'))
            repo.remotes.origin.pull(progress=CloneProgress())
        else:
            repo = git.Repo.clone_from(config.THEMEREPO, os.path.join(config.CACHEPATH,'theme_repo'), progress=CloneProgress())
        async with aiohttp.ClientSession() as session:
            for repo in progress.track(repo.iter_submodules(),total=len(repo.submodules),description='Parsing themes'):
                l.append(await Git.get_partial(*repo.url.split('/')[-2:],session=session))
        return l

    async def get_partial(user,repo,branch='master',session=None):
        if not session:
            async with aiohttp.ClientSession() as session:
                return await Git.get_partial(user,repo,branch=branch,session=session)
        async with session.get(f'https://raw.githubusercontent.com/{user}/{repo}/{branch}/theme.toml') as resp:
            if branch != 'main' and resp.status == 404:
                print('using main branch')
                return await Git.get_partial(session,user,repo,branch='main')
            return await PartialTheme.from_toml(toml.loads(await resp.text()),f'https://github.com/{user}/{repo}/tree/{branch}')

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
            console=console.Console(theme=self.theme),
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
    
    


