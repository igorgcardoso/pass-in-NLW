from sqlalchemy import create_engine, Engine
from sqlalchemy.orm import sessionmaker
from threading import Lock


class __SingletonMeta(type):
    _instances = {}
    _lock: Lock = Lock()

    def __call__(cls, *args, **kwargs):
        with cls._lock:
            if cls not in cls._instances:
                instance = super().__call__(*args, **kwargs)
                cls._instances[cls] = instance
        return cls._instances[cls]


class DBConnectionHandler(metaclass=__SingletonMeta):
    def __init__(self):
        self.__connection_string = "sqlite:///storage.db"
        self.__engine = create_engine(self.__connection_string)

    def get_engine(self) -> Engine:
        return self.__engine

    def __enter__(self):
        session_maker = sessionmaker()
        self.session = session_maker(bind=self.__engine)
        return self

    def __exit__(self, exc_type, exc_val, exc_tb):
        if self.session is not None:
            self.session.close()
