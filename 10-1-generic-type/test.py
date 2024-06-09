from enum import Enum


class Radar(Enum):
    REAR = (0, 1, "Rear")
    FRONT = (1, 2, "Front")
    LEFT = (2, 3, "Left")
    RIGHT = (3, 4, "Right")

    def __init__(self, index, type, topic):
        self.index = index
        self.type = type
        self.topic = topic


print(Radar.REAR.topic)
