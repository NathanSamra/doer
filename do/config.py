from typing import List

from do import storage


def context() -> str:
    config = storage.config()
    if config.has_option('database', 'context'):
        return config.get('database', 'context')

    return 'default'


def contexts() -> List[str]:
    return [item.name for item in storage.database().iterdir() if item.is_dir()]


def set_context(new_context: str):
    config = storage.config()
    if not config.has_section('database'):
        config.add_section('database')

    config.set('database', 'context', new_context)
    storage.set_config(config)
