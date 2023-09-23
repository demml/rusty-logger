from fastapi import FastAPI
from router import api_router


class FastApp:
    def __init__(self):
        self.port = 8888
        self.app = FastAPI(title="MY_APP")

    def get_app(self):
        self.app.include_router(api_router)

        return self.app


def run_app():
    return FastApp().get_app()


if __name__ == "__main__":
    run_app()
