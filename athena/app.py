from flask import Flask
from dotenv import load_dotenv, find_dotenv
import logging
import sys
from routes.embedding import embedding_routes
from routes.llamaindex_chat import llamaindex_routes
from routes.langchain_bot import langchain_routes
from routes.chat_bot import chat_bot_routes
from routes.pg_nlsql import pg_nlsql_routes


logging.basicConfig(stream=sys.stdout, level=logging.DEBUG)
logging.getLogger().addHandler(logging.StreamHandler(stream=sys.stdout))

load_dotenv()

app = Flask(__name__)

embedding_routes(app)
chat_bot_routes(app)
langchain_routes(app)
llamaindex_routes(app)
pg_nlsql_routes(app)

if __name__ == '__main__':
    print("Starting Athena...")
    app.run(host='0.0.0.0', debug=True, port=3005)
