import time

def string_today(timer):
    now = timer.time()
    now_struct = time.localtime(now)
    return time.strftime("%Y-%m-%d", now_struct)

if __name__ == "__main__":
    class Timer:
        def time(self):
            return time.time()

    print(string_today(Timer()))
