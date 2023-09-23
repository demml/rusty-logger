gunicorn \
      -k uvicorn.workers.UvicornWorker \
      --bind=0.0.0.0:8888 \
      --workers 4 \
      "main:run_app()"