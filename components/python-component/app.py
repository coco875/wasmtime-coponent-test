import wit_world
from wit_world.imports import iactor

class WitWorld(wit_world.WitWorld):
    def init(self) -> None:
        print("Python component initialized")

    def actor_init(self, a: iactor.Actor, x: float, y: float) -> None:
        print(f"Python actor init: x={x}, y={y}")
        a.set_x(x)
        a.set_y(y)

    def actor_update(self, a: iactor.Actor) -> None:
        print("Python actor update")

    def actor_render(self, a: iactor.Actor) -> None:
        print("Python actor render")