"""Stubs for Rust functions."""

from dataclasses import dataclass
from typing import Protocol

class StateMachine(Protocol): ...

@dataclass(frozen=True)
class Transition(Protocol): ...
