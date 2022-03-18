from configparser import ConfigParser
from pathlib import Path

from appdirs import AppDirs

from do import metadata


def _dirs():
    return AppDirs(appname=metadata.name, appauthor=metadata.author)


def database() -> Path:
    database_loc = Path(_dirs().user_data_dir)
    if not database_loc.exists():
        database_loc.mkdir(parents=True, exist_ok=True)
    return database_loc


def config():
    config_file = Path(_dirs().user_config_dir, 'config.ini')
    if not config_file.exists():
        dir_ = config_file.parent
        dir_.mkdir(parents=True, exist_ok=True)
        config_file.touch()

    config_ = ConfigParser()
    with config_file.open('r') as f:
        config_.read_file(f)

    return config_
