from flask import render_template, request
from llama_index import SQLDatabase
from llama_index.indices.struct_store import (
    NLSQLTableQueryEngine,
)
from llama_index.indices.struct_store.sql_query import NLSQLTableQueryEngine
from sqlalchemy import create_engine

def pg_nlsql_routes(app, connection_string):

    query_engine = create_nlsql_query_engine(connection_string)

    @app.route("/nlsql")
    def home_nlsql():
        return render_template("index.html", agent="Postgress Natural Language SQL Query", get_endpoint="/get_nlsql")

    @app.route("/get_nlsql")
    def get_nlsql_bot_response():
        userText = request.args.get('msg')
        response = query_engine.query(userText)
        return f"{response.response} \n \t  SQL = {response.metadata['sql_query']}"


def create_nlsql_query_engine(connection_string):
    engine = create_engine(connection_string)
    sql_database = SQLDatabase(engine)
    # Create a structured store to offer a context to GPT
    query_engine = NLSQLTableQueryEngine(sql_database)
    return query_engine
