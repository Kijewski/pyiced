from pyiced import (
    Align, checkbox, CheckboxStyle, CheckboxStyleSheet, Color, container,
    IcedApp, Length,
)


class CheckboxExample(IcedApp):
    class settings:
        class window:
            size = (640, 320)

    def __init__(self):
        self.__is_checked = False

    def title(self):
        if self.__is_checked:
            return 'A checked checkbox'
        else:
            return 'A checkbox'

    def view(self):
        styled_checkbox = checkbox(
            'set',
            self.__is_checked,
            self.title(),
            style=CheckboxStyleSheet(
                active=CheckboxStyle(
                    'active',
                    background=Color(0.64, 0.41, 0.32),
                ),
                active_checked=CheckboxStyle(
                    'active_checked',
                    background=Color(0, 0.71, 0.296),
                ),
            ),
        )
        return container(
            styled_checkbox,
            padding=20, align_x=Align.CENTER, align_y=Align.CENTER,
            width=Length.FILL, height=Length.FILL,
        )

    def update(self, msg, clipboard):
        match msg:
            case 'set', is_checked:
                self.__is_checked = is_checked


if __name__ == '__main__':
    CheckboxExample().run()
