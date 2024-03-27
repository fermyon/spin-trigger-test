from typing import List
import spin_test.exports
from spin_test.imports.test_types import Err, ErrorOther
import add_test
import basic_test
import inspect

separator = "/"

class SpinTestHandler(spin_test.exports.SpinTestHandler):
    def get_test_list(self) -> List[str]:
        # Get all modules in the current global scope
        modules = [obj for name, obj in globals().items() if inspect.ismodule(obj)]

        # Collect test functions from all modules
        test_functions = []
        for module in modules:
            for name, func in inspect.getmembers(module, predicate=callable):
                if name.startswith("test_"):
                     test_functions.append(f"{module.__name__}{separator}{name}")

        return test_functions
    
    def execute_test(self, full_function_name: str) -> None:
        module_name, function_name = full_function_name.split(separator, 1)

        # Import the module dynamically based on the module name
        module = __import__(module_name)

        if hasattr(module, function_name) and callable(getattr(module, function_name)):
            func = getattr(module, function_name)
            try:
                func()
            except Exception as e:
                raise Err(ErrorOther(str(e)))
        else:
            print(f"Function '{function_name}' not found in module '{module_name}'.")
        
