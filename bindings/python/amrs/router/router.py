import abc

from amrs import config

class ModelInfo:
    request_num: int = 0
    average_latency: float = 0.0

class Router(abc.ABC):
    def __init__(self, model_list: list[config.ModelID]):
        self._model_list = model_list

    @abc.abstractmethod
    def sample(self, content: str) -> config.ModelID:
        pass

def new_router(model_cfgs: list[config.ModelConfig], mode: config.RoutingMode) -> Router:
    model_list = [f"{model_cfg.provider}/{model_cfg.id}" for model_cfg in model_cfgs]

    if mode == config.RoutingMode.RANDOM:
        from amrs.router.random import RandomRouter
        return RandomRouter(model_list)
    else:
        raise ValueError(f"Unknown routing mode: {mode}")