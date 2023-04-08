import os
import requests
import toml

class Theme:
    name:str
    desc: str
    repo: str
    author: str
    version: str
    subthemes: 'list[Theme]'
    default_subtheme: str
    depends: 'list[str]'
    _repo: str
    _path: os.path

    async def from_toml(theme:dict[str,any],_repo=None):
        t = Theme()
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
            t.subthemes.append(await Theme.from_toml(toml.loads(raw.text),t._repo))
        return t




    


        
