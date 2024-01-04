from flask import render_template, request
from llama_index import SQLDatabase
from llama_index.indices.struct_store import (
    NLSQLTableQueryEngine,
)
from llama_index.indices.struct_store.sql_query import NLSQLTableQueryEngine
import os
from sqlalchemy import create_engine

RDS_HOSTNAME = os.getenv('RDS_HOSTNAME')
RDS_PORT = os.getenv('RDS_PORT')
RDS_DB_NAME = os.getenv('RDS_DB_NAME')
RDS_USERNAME = os.getenv('RDS_USERNAME')
RDS_PASSWORD = os.getenv('RDS_PASSWORD')

model_3_5 = "gpt-3.5-turbo"
model_4_t = "gpt-4-1106-preview"


def pg_nlsql_routes(app):

    query_engine = create_nlsql_query_engine()

    @app.route("/nlsql")
    def home_nlsql():
        return render_template("index.html", agent="Postgress Natural Language SQL Query", get_endpoint="/get_nlsql")

    @app.route("/get_nlsql")
    def get_nlsql_bot_response():
        userText = request.args.get('msg')
        response = query_engine.query(userText)
        return f"{response.response} \n \t  SQL = {response.metadata['sql_query']}"


def create_nlsql_query_engine():
    connection_string = f"postgresql://{RDS_USERNAME}:{RDS_PASSWORD}@db:{RDS_PORT}/{RDS_DB_NAME}?sslmode=disable"
    engine = create_engine(connection_string)
    sql_database = SQLDatabase(engine)
    # Create a structured store to offer a context to GPT
    query_engine = NLSQLTableQueryEngine(sql_database)
    return query_engine
