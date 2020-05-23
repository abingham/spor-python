from spor import open_repository


def test_cosmic_ray_filtering(repo_path):
    "A rough acceptance test for cosmic ray"

    module_path = repo_path / 'file1.py'

    repo = open_repository(module_path)

    for anchor_id, anchor in repo.items():
        assert anchor.file_path == module_path.absolute()
        metadata = anchor.metadata
        assert isinstance(metadata["mutate"], bool)

    assert len(list(repo.items())) == 2
