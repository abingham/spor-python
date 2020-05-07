# This is our initial goal for spor-python.
#
# This represents the API we need to support in order to support cosmic-ray.

from spor.fs_repository import initialize, open_repository
from spor.anchor import Anchor, Context
import pytest


def test_open_raises_OSError_if_repo_is_missing(tmp_path):
    tmp_file = tmp_path / 'temp.py'
    tmp_file.write_text('# empty')
    with pytest.raises(OSError):
        open_repository(tmp_file)


def test_open_works_if_repo_exists(tmp_path):
    initialize_repo(tmp_path)
    open_repository(tmp_path)


def test_canned(tmp_path):
    "A rough acceptance test for cosmic ray"

    initialize_repo(tmp_path)

    module_path = tmp_path / 'file1.py'

    repo = open_repository(module_path)

    for anchor_id, anchor in repo.items():
        assert anchor.file_path == module_path.absolute()
        metadata = anchor.metadata
        assert isinstance(metadata["mutate"], bool)

    assert len(list(repo.items())) == 2


def initialize_repo(tmp_path):
    initialize(tmp_path)

    repo = open_repository(tmp_path)

    file_1 = tmp_path / 'file1.py'
    text = 'def foo(x): return x * 2'
    file_1.write_text(text)

    context = Context(text, 5, 6, 5)
    anchor = Anchor(str(file_1), context, {'mutate': True}, 'utf-8')
    repo.add(anchor)

    context = Context(text, 10, 2, 3)
    anchor = Anchor(str(file_1), context, {'mutate': False}, 'utf-8')
    repo.add(anchor)

