from datetime import datetime, timedelta

from pyiced import column, every, Message, IcedApp, text


class SubscriptionExample(IcedApp):
    def __init__(self):
        self.__counter = 0
        self.__ts = self.__last = datetime.now()

    class settings:
        class window:
            size = (320, 64)

    def title(self):
        return 'Subscription Example'

    def view(self):
        duration = (self.__ts - self.__last).microseconds
        return column([
            text(f'Counter: {self.__counter:09d}'),
            text(f'Duration: {duration:06d} ms'),
            text(f'Timestamp: {self.__ts}'),
        ])

    def subscriptions(self):
        return [
            every(timedelta(milliseconds=10), Message('tick')),
        ]

    def update(self, msg):
        match msg:
            case Message('tick'):
                self.__counter += 1
                self.__last = self.__ts
                self.__ts = datetime.now()


if __name__ == '__main__':
    SubscriptionExample().run()
