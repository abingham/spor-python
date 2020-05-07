# This is our initial goal for spor-python.
#
from spor.fs_repository import open_repository
import pytest


def test_open_raises_OSError_if_repo_is_missing(tmp_path):
    tmp_file = tmp_path / 'temp.py'
    tmp_file.write_text('# empty')
    with pytest.raises(OSError):
        open_repository(tmp_file)


def test_open_works_if_repo_exists(repo_path):
    open_repository(repo_path)


