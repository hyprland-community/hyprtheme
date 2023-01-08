import os
import inspect
import json

import rich.console

config_dir = os.path.expanduser('~/.config/hypr/')
theme_dir = config_dir + 'themes/'

console = rich.console.Console()

class Theme:
    def __init__(self, name, path, desc=None, author=None, version=None, repo=None):
        self.author = author
        self.version = version
        self.repo = repo
        self.desc = desc
        self.name = name
        self.path = path
        self.subthemes = []
        with open(path) as f:
            self.raw = f.read()

    def parse(self):
        for line in self.raw.splitlines():
            line = line.strip()
            if line.startswith('#'):
                line = line[1:].strip()
                keys = list(map(lambda x: x.strip(),line.split('=',1)))
                attrs = [ i[0] for i in inspect.getmembers(self, lambda a:not(inspect.isroutine(a))) if not(i[0].startswith('__') and i[0].endswith('__'))]
                print(keys[0],attrs)
                if keys[0] in attrs:
                    setattr(self, keys[0], keys[1])
                if keys[0] == 'subtheme':
                    dat = json.loads(keys[1])
                    self.subthemes.append(
                        Theme(
                            dat['name'],
                            os.path.expanduser(dat['path']).replace(
                                './',
                                '/'.join(self.path.split('/')[:-1]) + '/'
                            ),
                            dat.get('desc') or self.desc,
                            dat.get('author') or self.author,
                            dat.get('version') or self.version,
                            dat.get('repo') or self.repo
                        ))
    
    def to_dict(self):
        return {
            'author': self.author,
            'version': self.version,
            'repo': self.repo,
            'name': self.name,
            'path': self.path,
            'subthemes': [theme.to_dict() for theme in self.subthemes]
        }

class HyprTheme:
    def __init__(self):
        self.themes = []

    def list_themes(self):
        for theme in os.listdir(theme_dir):
            if os.path.isdir(theme_dir + theme):
                self.themes.append(Theme(theme, theme_dir + theme + '/theme.conf'))

    def parse_themes(self):
        for theme in self.themes:
            theme.parse()

if __name__ == '__main__':
    hypr = HyprTheme()
    hypr.list_themes()
    hypr.parse_themes()
    for theme in hypr.themes:
        console.log(theme.to_dict())

