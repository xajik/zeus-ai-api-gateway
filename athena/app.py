from flask import Flask
from dotenv import load_dotenv, find_dotenv
import logging
import sys
from routes.embedding import embedding_routes
from routes.llamaindex_chat import llamaindex_routes
from routes.langchain_bot import langchain_routes
from routes.chat_bot import chat_bot_routes
from routes.pg_nlsql import pg_nlsql_routes
from routes.react_llamaindex import react_llamaindex_routes
import os

logging.basicConfig(stream=sys.stdout, level=logging.DEBUG)
logging.getLogger().addHandler(logging.StreamHandler(stream=sys.stdout))

load_dotenv()

RDS_HOSTNAME = os.getenv('RDS_HOSTNAME')
RDS_PORT = os.getenv('RDS_PORT')
RDS_DB_NAME = os.getenv('RDS_DB_NAME')
RDS_USERNAME = os.getenv('RDS_USERNAME')
RDS_PASSWORD = os.getenv('RDS_PASSWORD')

connection_string = f"postgresql://{RDS_USERNAME}:{RDS_PASSWORD}@db:{RDS_PORT}/{RDS_DB_NAME}?sslmode=disable"
print("Connection string:", connection_string)

app = Flask(__name__)

embedding_routes(app)
chat_bot_routes(app)
langchain_routes(app)
llamaindex_routes(app, connection_string)
pg_nlsql_routes(app, connection_string)
react_llamaindex_routes(app, connection_string)

if __name__ == '__main__':
    print("Starting Athena...")
    app.run(host='0.0.0.0', debug=True, port=3005)
