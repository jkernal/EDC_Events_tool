from pathlib import Path
from tomllib import load

def load_config(path: str | Path) -> dict:
    """Load TOML file into a Python dict and return it."""
    path = Path(path)
    with path.open("rb") as f:
        return load(f)
