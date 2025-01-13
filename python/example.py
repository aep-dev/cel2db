import sys
import os
import importlib.util

# boilerplate to load the python module
so_path = os.path.realpath(os.path.join(os.path.dirname(__file__), 'libcel2db_py.so'))
spec = importlib.util.spec_from_file_location('cel2db_py', so_path)
module = importlib.util.module_from_spec(spec)
spec.loader.exec_module(module)
sys.modules['cel2db_py'] = module

# use the module
from cel2db_py import cel_to_db
print(cel_to_db("1 + 2", "sqlite"))
