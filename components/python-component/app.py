import example

class Example(example.Example):
    def add(self, x: int, y: int) -> int:
        return x+y

    def test(self) -> example.Customer:
        return example.Customer(3, "cc")