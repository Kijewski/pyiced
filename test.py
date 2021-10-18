import pyiced.pyiced


class X:
    def title(self):
        return "TEST 1323"

    def update(self, message):
        global value

        match message:
            case pyiced.pyiced.Message(python=('v', v)):
                if value != v:
                    print('v', v)
                    value = v

            case pyiced.pyiced.Message(window='filedropped', file=file):
                print('filedropped', repr(file))

            case pyiced.pyiced.Message(native='window', window=window):
                print('window', window)

            case pyiced.pyiced.Message(native='keyboard', keyboard=keyboard):
                print('keyboard', keyboard)

        return []

    def new(self):
        return []

    def should_exit(self):
        return False

    def view(self):
        return pyiced.pyiced.slider(s, 0, 10, value, lambda v: pyiced.pyiced.Message(('v', v)))


s = pyiced.pyiced.SliderState()
value = 5

pyiced.run_iced(X())
