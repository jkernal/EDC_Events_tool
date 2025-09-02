import src.ImportEvents as ImportEvents

from ImportEvents import manages_files

def test_manages_files(tmp_path, monkeypatch):
    template_dir = tmp_path.parent / 'template'
    template_dir.mkdir()
    tpl = template_dir / 'temp.xlsx'
    tpl.write_text('')
    loader_bins = tmp_path / 'loader' / 'bins'
    loader_bins.mkdir(parents=True)
    toyopath = loader_bins / 'toyo_comments.bin'
    toyopath.write_text('')
    swpath = loader_bins / 'sw_comments.bin'
    swpath.write_text('')
    monkeypatch.setattr(ImportEvents, 'WRK_DIR', tmp_path)
    locations = manages_files()
    assert locations[0] == tpl
    assert locations[1] == tmp_path / f"out_{tpl.name}"
    assert locations[2] == toyopath
    assert locations[3] == swpath