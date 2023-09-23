from fastapi import APIRouter
from pydantic import BaseModel
from logger import ApiLogger

router = APIRouter()

logger = ApiLogger.get_logger(__file__)


class HealthCheckResult(BaseModel):
    is_alive: bool


@router.get("/healthcheck", response_model=HealthCheckResult, name="healthcheck")
def get_healthcheck() -> HealthCheckResult:
    logger.info("healthcheck")
    return HealthCheckResult(is_alive=True)
