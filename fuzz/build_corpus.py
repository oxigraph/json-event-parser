import hashlib
import random
from pathlib import Path

base = Path(__file__).parent.parent
target_dir = base / "fuzz" / "corpus" / "parse"
target_dir.mkdir(parents=True, exist_ok=True)
for f in base.rglob("*.json"):
    for _ in range(3):
        data = f.read_bytes()
        pos = random.randint(0, len(data))
        data = data[:pos] + b"\xff" + data[pos:]
        hash = hashlib.sha256()
        hash.update(data)
        (target_dir / hash.hexdigest()).write_bytes(data)
