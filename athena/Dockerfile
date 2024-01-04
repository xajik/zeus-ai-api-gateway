FROM python:3.9-slim

WORKDIR /app

COPY ./data /data

RUN pip install --upgrade pip

COPY requirements.txt requirements.txt
RUN pip install -r requirements.txt

EXPOSE 3005

CMD ["python", "app.py"]
