from .spor import align, anchor, repository  # noqa: F401


Anchor = anchor.Anchor
Context = anchor.Context


def initialize_repository(path):
    return repository.initialize(str(path))


def open_repository(path):
    return repository.Repository(str(path))
