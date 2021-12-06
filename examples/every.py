from datetime import datetime, timedelta

from pyiced import column, every, IcedApp, Instant, text


class SubscriptionExample(IcedApp):
    def __init__(self):
        self.__counter = 0
        self.__instant = Instant()
        self.__last_instant = self.__instant
        self.__ts = datetime.now().time()
        self.__subscription = every(timedelta(milliseconds=16.667), 'tick')

    class settings:
        class window:
            size = (320, 64)

    def title(self):
        return 'Subscription Example'

    def view(self):
        duration = self.__instant - self.__last_instant
        return column([
            text(f'Counter: {self.__counter:09d}'),
            text(f'Duration: {duration * 10**3:9.6f} ms'),
            text(f'Time: {self.__ts}')
        ])

    def subscriptions(self):
        return [self.__subscription]

    def update(self, msg, clipboard):
        match msg:
            case ('tick', instant):
                self.__last_instant = self.__instant
                self.__counter += 1
                self.__instant = instant
                self.__ts = datetime.now().time()


if __name__ == '__main__':
    SubscriptionExample().run()
