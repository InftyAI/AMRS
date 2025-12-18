import random

from amrs.config import ModelID
from amrs.router.router import Router

class RandomRouter(Router):
    def __init__(self, model_list: list[ModelID]):
        super().__init__(model_list)

    def sample(self, _: str) -> ModelID:
        return random.choice(self._model_list)
