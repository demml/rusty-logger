from fastapi import APIRouter
from pydantic import BaseModel
from logger import ApiLogger

router = APIRouter()

logger = ApiLogger.get_logger(__file__)


class LogResult(BaseModel):
    success: bool


arg1 = {"key": 10}
arg2 = "arg2"


@router.get("/log", response_model=LogResult, name="log")
def get_healthcheck() -> LogResult:
    logger.info("Logging test arg1:{} arg2:{}", arg1, arg2)
    return LogResult(success=True)
