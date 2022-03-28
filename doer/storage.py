from configparser import ConfigParser
from pathlib import Path

from appdirs import AppDirs

from doer import metadata

_dirs = AppDirs(appname=metadata.name, appauthor=metadata.author)
_config_file = Path(_dirs.user_config_dir, 'config.ini')


def database() -> Path:
    database_loc = Path(_dirs.user_data_dir)
    if not database_loc.exists():
        database_loc.mkdir(parents=True, exist_ok=True)
    return database_loc


def config():
    if not _config_file.exists():
        dir_ = _config_file.parent
        dir_.mkdir(parents=True, exist_ok=True)
        _config_file.touch()

    config_ = ConfigParser()
    with _config_file.open('r') as f:
        config_.read_file(f)

    return config_


def set_config(config_: ConfigParser):
    with _config_file.open('w+') as f:
        config_.write(f)
