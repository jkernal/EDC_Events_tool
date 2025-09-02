import src.ImportEvents as ImportEvents

from pathlib import Path
from ImportEvents import load_shm_as_dict

def test_shm():
    
    test_toyo_data_file = Path(__file__).parent / "data" / "test_toyo_comments.bin"
    test_sw_data_file = Path(__file__).parent / "data" / "test_sw_comments.bin"
    
    toyo_dict = load_shm_as_dict(test_toyo_data_file)
    sw_dict = load_shm_as_dict(test_sw_data_file)
    
    assert toyo_dict.get("GMF900") == "FAULT B1"
    assert sw_dict.get("GMF900") == "異常PL1/FAULT B1"