import pytest

from spor.anchor import Anchor, Context
from spor.fs_repository import initialize, open_repository


@pytest.fixture
def repo_path(tmp_path):
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

    return tmp_path