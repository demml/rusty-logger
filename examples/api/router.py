from fastapi import APIRouter
import healthcheck
import routes

api_router = APIRouter(responses={404: {"description": "Not found"}})
api_router.include_router(healthcheck.router, tags=["health"])
api_router.include_router(routes.router, tags=["routes"])
