import tomllib
from pathlib import Path
import sys

sys.path.insert(0, str(Path(__file__).resolve().parents[1] / "python"))

project_metadata = tomllib.loads(
    (Path(__file__).resolve().parents[1] / "pyproject.toml").read_text(
        encoding="utf-8"
    )
)

project = "Quentangled"
copyright = "2026, Quentangled contributors"
author = "Quentangled contributors"
release = project_metadata["project"]["version"]

extensions = [
    "myst_parser",
    "sphinx.ext.autodoc",
    "sphinx.ext.napoleon",
    "sphinx.ext.viewcode",
]

source_suffix = {
    ".md": "markdown",
    ".rst": "restructuredtext",
}
master_doc = "index"
exclude_patterns = ["_build"]
html_theme = "sphinx_rtd_theme"
html_title = "Quentangled documentation"
html_static_path = []

autodoc_member_order = "bysource"
autodoc_typehints = "description"
autodoc_inherit_docstrings = True
nitpicky = True
