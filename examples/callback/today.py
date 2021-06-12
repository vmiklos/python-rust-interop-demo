import time

import rust

string_today = rust.py_string_today
ChronoTimer = rust.PyChronoTimer

if __name__ == "__main__":
    print(string_today(ChronoTimer()))
