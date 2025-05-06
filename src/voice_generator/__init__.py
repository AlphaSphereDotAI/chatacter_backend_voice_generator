"""
Core constants and initialization for the Voice Generator package.

This module sets up the voice generation models, pipelines, and loads
necessary data for the application.
"""

from pathlib import Path
from os import getenv
from kokoro import KPipeline
from torch import cuda
from dotenv import load_dotenv
from loguru import logger

load_dotenv()

BASE_DIR: Path = Path(__file__).parent.parent.parent
DEBUG: bool = getenv(key="DEBUG", default="False").lower() == "true"
CUDA_AVAILABLE: bool = cuda.is_available()
CHAR_LIMIT: int = 5000
PIPELINE: KPipeline = KPipeline(lang_code="a")

logger.info(f"CUDA Avalible: {CUDA_AVAILABLE}")

try:
    with open(BASE_DIR / "en.txt", "r", encoding="utf-8") as r:
        random_quotes: list[str] = [line.strip() for line in r]
except FileNotFoundError as e:
    raise FileNotFoundError(f"Missing required text file: {BASE_DIR / 'en.txt'}") from e

CHOICES: dict[str, str] = {
    "🇺🇸 🚺 Heart ❤️": "af_heart",
    "🇺🇸 🚺 Bella 🔥": "af_bella",
    "🇺🇸 🚺 Nicole 🎧": "af_nicole",
    "🇺🇸 🚺 Aoede": "af_aoede",
    "🇺🇸 🚺 Kore": "af_kore",
    "🇺🇸 🚺 Sarah": "af_sarah",
    "🇺🇸 🚺 Nova": "af_nova",
    "🇺🇸 🚺 Sky": "af_sky",
    "🇺🇸 🚺 Alloy": "af_alloy",
    "🇺🇸 🚺 Jessica": "af_jessica",
    "🇺🇸 🚺 River": "af_river",
    "🇺🇸 🚹 Michael": "am_michael",
    "🇺🇸 🚹 Fenrir": "am_fenrir",
    "🇺🇸 🚹 Puck": "am_puck",
    "🇺🇸 🚹 Echo": "am_echo",
    "🇺🇸 🚹 Eric": "am_eric",
    "🇺🇸 🚹 Liam": "am_liam",
    "🇺🇸 🚹 Onyx": "am_onyx",
    "🇺🇸 🚹 Santa": "am_santa",
    "🇺🇸 🚹 Adam": "am_adam",
    "🇬🇧 🚺 Emma": "bf_emma",
    "🇬🇧 🚺 Isabella": "bf_isabella",
    "🇬🇧 🚺 Alice": "bf_alice",
    "🇬🇧 🚺 Lily": "bf_lily",
    "🇬🇧 🚹 George": "bm_george",
    "🇬🇧 🚹 Fable": "bm_fable",
    "🇬🇧 🚹 Lewis": "bm_lewis",
    "🇬🇧 🚹 Daniel": "bm_daniel",
}

TOKEN_NOTE = """
💡 Customize pronunciation with Markdown link syntax and /slashes/ like `[Kokoro](/kˈOkəɹO/)`

💬 To adjust intonation, try punctuation `;:,.!?—…"()""` or stress `ˈ` and `ˌ`

⬇️ Lower stress `[1 level](-1)` or `[2 levels](-2)`

⬆️ Raise stress 1 level `[or](+2)` 2 levels (only works on less stressed, usually short words)
"""

_STREAM_NOTE: list[str] = [
    "⚠️ There is an unknown Gradio bug that might yield no audio the first time you click `Stream`."
]
if CHAR_LIMIT is not None:
    _STREAM_NOTE.append(f"✂️ Each stream is capped at {CHAR_LIMIT} characters.")
    _STREAM_NOTE.append(
        "🚀 Want more characters? You can [use Kokoro directly](https://huggingface.co/hexgrad/Kokoro-82M#usage) or duplicate this space:"
    )
STREAM_NOTE: str = "\n\n".join(_STREAM_NOTE)
