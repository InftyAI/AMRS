import random

from amrs.config import ModelName
from amrs.router.router import Router

class RandomRouter(Router):
    def __init__(self, model_list: list[ModelName]):
        super().__init__(model_list)

    def sample(self, _: str) -> ModelName:
        return random.choice(self._model_list)
