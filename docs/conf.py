import os
import re
import sys

sys.path.insert(0, os.path.abspath('..'))

import pyiced
import pyiced.css_color


needs_sphinx = '4.2'

extensions = [
    'myst_parser',
    'sphinx.ext.autodoc',
    'sphinx.ext.autosectionlabel',
    'sphinx.ext.autosummary',
    'sphinx.ext.napoleon',
    'sphinx.ext.intersphinx',
    'sphinx_autodoc_typehints',
    'sphinxprettysearchresults',
]

display_toc = True
autodoc_default_flags = ['members']
autosummary_generate = True
napoleon_google_docstring = False
autosectionlabel_prefix_document = True
templates_path = ['_templates']
source_suffix = '.rst'
master_doc = 'index'
language = 'en'
exclude_patterns = []
pygments_style = 'sphinx'
todo_include_todos = False
autoclass_content = 'both'

html_theme = 'cloud'
html_static_path = ['_static']
html_css_files = [
    'custom.css',
]
html_sidebars = {
    '**': [
        'globaltoc.html',
        'searchbox.html',
    ]
}

project = 'PyIced'
copyright = '2021, René Kijewski'
author = 'René Kijewski'
htmlhelp_basename = 'PyIceddoc'

release = pyiced.__version__
version = re.match(r'\A\d+\.\d+\.\d+', release).group(0)

latex_documents = [
    (master_doc, 'PyIced.tex', 'PyIced Documentation', author, 'manual'),
]
man_pages = [
    (master_doc, 'pyiced', 'PyIced Documentation', [author], 1)
]

texinfo_documents = [
    (master_doc, 'PyIced', 'PyIced Documentation', author, 'PyIced', 'One line description of project.', 'Miscellaneous'),
]

intersphinx_mapping = {
    'python': ('https://docs.python.org/3.10', None),
}
