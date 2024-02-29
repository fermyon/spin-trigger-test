from typing import TypeVar, Generic, Union, Optional, Union, Protocol, Tuple, List, Any, Self
from enum import Flag, Enum, auto
from dataclasses import dataclass
from abc import abstractmethod
import weakref

from .types import Result, Ok, Err, Some



class SpinTest(Protocol):

    @abstractmethod
    def get_test_list(self) -> List[str]:
        """
        Raises: `spin_test.types.Err(spin_test.imports.test_types.Error)`
        """
        raise NotImplementedError

    @abstractmethod
    def execute_test(self, name: str) -> None:
        """
        Raises: `spin_test.types.Err(spin_test.imports.test_types.Error)`
        """
        raise NotImplementedError

