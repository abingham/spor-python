import pytest

from spor import Anchor, Context, initialize_repository, open_repository


@pytest.fixture
def repo_path(tmp_path):
    initialize_repository(tmp_path)

    repo = open_repository(tmp_path)

    file_1 = tmp_path / 'file1.py'
    text = 'def foo(x): return x * 2'
    file_1.write_text(text)

    context = Context.from_text(text, 5, 6, 5)
    anchor = Anchor(str(file_1), context, {'mutate': True}, 'utf-8')
    repo.add(anchor)

    context = Context.from_text(text, 10, 2, 3)
    anchor = Anchor(str(file_1), context, {'mutate': False}, 'utf-8')
    repo.add(anchor)

    return tmp_path
