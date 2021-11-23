from pyiced import (
    Align, container, Message, IcedApp, Length, Subscription, text,
)


class FullscreenExample(IcedApp):
    def __init__(self):
        self.__fullscreen = False

    class settings:
        class window:
            size = (640, 320)

    def fullscreen(self):
        return self.__fullscreen

    def subscriptions(self):
        return [Subscription.UNCAPTURED]

    def title(self):
        return self.__message

    def update(self, msg, clipboard):
        match msg:
            case Message(keyboard='keyreleased', key_code='F11'):
                self.__fullscreen = not self.__fullscreen

    def view(self):
        return container(
            text(self.__message, size=40),
            padding=20, align_x=Align.CENTER, align_y=Align.CENTER,
            width=Length.FILL, height=Length.FILL,
        )

    @property
    def __message(self):
        if self.__fullscreen:
            return 'Fullscreen (press F11!)'
        else:
            return 'Windowed (press F11!)'


if __name__ == '__main__':
    FullscreenExample().run()
