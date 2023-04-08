import os
import requests
import subprocess
import toml

class Script:
    load: os.path
    unload: os.path
    update: os.path

    def __init__(self, load:os.path=None, unload:os.path=None, update:os.path=None):
        self.load = load
        self.unload = unload
        self.update = update

class Component:
    name: str
    desc: str
    script: Script

    def __init__(self, name:str, desc:str, script:Script):
        self.name = name
        self.desc = desc
        self.script = script
    
    @classmethod
    async def from_toml(cls, component:dict[str,any], name:str=None):
        return cls(
            name=name or component.get('name'),
            desc=component.get('desc') or component.get('description'),
            script=Script(
                load=component.get('load'),
                unload=component.get('unload'),
                update=component.get('update')
            )
        )

    def load(self):
        p = subprocess.run(os.path.abspath(self.script.load))
        print(os.path.abspath(self.script.load))
        return p.returncode
    
    def unload(self):
        p = subprocess.run(os.path.abspath(self.script.unload))
        return p.returncode

    def update(self):
        p = subprocess.run(os.path.abspath(self.script.update))
        return p.returncode



class PartialTheme:
    name:str
    desc: str
    repo: str
    author: str
    version: str
    subthemes: 'list[PartialTheme]'
    default_subtheme: str
    depends: 'list[str]'
    raw: dict[str,any]
    _repo: str
    _path: os.path

    async def from_toml(theme:dict[str,any],_repo=None):
        t = PartialTheme()
        t.raw = theme
        t.name = theme.get('theme').get('name')
        t.desc = theme.get('theme').get('description') or theme.get('theme').get('desc')
        t.repo = theme.get('theme').get('repo')
        t.author = theme.get('theme').get('author')
        t.version = theme.get('theme').get('version')
        t.default_subtheme = theme.get('theme').get('default_subtheme')
        t.depends = theme.get('theme').get('depends')
        t._repo = _repo or t.repo

        t.subthemes = []
        for subtheme in theme.get('theme').get('subthemes') or []:
            branch,_,repo,user = t._repo.split('/')[::-1][:4]

            path = subtheme
            if path.startswith('/'):
                path = path[1:]
            elif path.startswith('./'):
                path = path[2:]

            raw = requests.get(f'https://raw.githubusercontent.com/{user}/{repo}/{branch}/{path}')
            t.subthemes.append(await PartialTheme.from_toml(toml.loads(raw.text),t._repo))

        return t
    
class Theme(PartialTheme):
    async def from_partial(partialtheme:PartialTheme):
        t = Theme(partialtheme)

        # TODO: somehow get the correct path to theme dir

        t.components = []
        if t.raw.get('component'):
            for name,component in t.raw.get('component').items():
                c = await Component.from_toml(component,name=name)
                t.components.append(c)

        return t
    




    


        
