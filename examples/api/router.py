from fastapi import APIRouter
import healthcheck

api_router = APIRouter(responses={404: {"description": "Not found"}})
api_router.include_router(healthcheck.router, tags=["health"])
