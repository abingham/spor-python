from spor.spor import fs_repository


def open_repository(module_path):
    return fs_repository.FSRepository(str(module_path))
