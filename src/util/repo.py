import git
import toml
import os
import base64
import json
import requests
from rich import progress, console
from rich.theme import Theme
from git import RemoteProgress
from . import config



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

    theme = Theme({
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
            
            

def download_theme(theme:str):
    if theme.startswith('git+'):
        repo = theme[4:]
        git.Repo.clone_from(repo, os.path.join(config.THEMEPATH,theme.split('/')[-1]) , progress=CloneProgress())
    else:
        raise NotImplementedError


def list_themes():
    l = []
    if os.path.exists(os.path.join(config.CACHEPATH,'theme_repo')):
        repo = git.Repo(os.path.join(config.CACHEPATH,'theme_repo'))
        repo.remotes.origin.pull(progress=CloneProgress())
    else:
        repo = git.Repo.clone_from(config.THEMEREPO, os.path.join(config.CACHEPATH,'theme_repo'), progress=CloneProgress())
    for repo in progress.track(repo.iter_submodules(),total=len(repo.submodules),description='Fetching themes'):
        l.append(get_theme_toml(*repo.url.split('/')[-2:]))
    return l

def get_theme_toml(user,repo):
    raw = json.loads(requests.get(f'https://api.github.com/repos/{user}/{repo}/contents/theme.toml').text)
    if raw.get('content',None) is None:
        print(raw)
        return {'theme':{'name':repo}}
    return toml.loads(base64.b64decode(raw.get('content',None)).decode('utf-8'))
    


