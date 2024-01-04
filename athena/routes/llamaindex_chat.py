from flask import render_template, request, jsonify
from llama_index import SimpleDirectoryReader, StorageContext
from llama_index.indices.vector_store import VectorStoreIndex
from llama_index.vector_stores import PGVectorStore
import os
import psycopg2
from sqlalchemy import make_url

RDS_HOSTNAME = os.getenv('RDS_HOSTNAME')
RDS_PORT = os.getenv('RDS_PORT')
RDS_DB_NAME = os.getenv('RDS_DB_NAME')
RDS_USERNAME = os.getenv('RDS_USERNAME')
RDS_PASSWORD = os.getenv('RDS_PASSWORD')

table_name = "llmama_rag_vector"


def llamaindex_routes(app):

    query_engine = llamaindex_query_engine()

    @app.route("/llamaindex")
    def home_llamaindex():
        return render_template("index.html", agent="Llamaindex", get_endpoint="/get_llamaindex")

    @app.route("/get_llamaindex")
    def get_llamaindex_bot_response():
        userText = request.args.get('msg')
        response = query_engine.query(userText)
        return str(response)
    
    @app.route("/injest_llamaindex", methods=['POST'])
    def injest_llamaindex():
        data = request.get_json()
        path = data.get('path')
        if not path:
            return jsonify({"error": "Missing 'path' parameter"}), 400        
        injest_by_path(path=path)
        return f"Ingested {path} in the RAG index"
        

def llamaindex_query_engine():
    vector_store = get_vector_store()
    index = VectorStoreIndex.from_vector_store(vector_store)
    query_engine = index.as_query_engine()
    return query_engine

def injest_by_path(path):
    documents = SimpleDirectoryReader(path).load_data()
    vector_store = get_vector_store()
    storage_context = StorageContext.from_defaults(vector_store=vector_store)
    query_engine = VectorStoreIndex.from_documents(documents, storage_context=storage_context)
    return query_engine

def get_vector_store():
    connection_string = f"postgresql://{RDS_USERNAME}:{RDS_PASSWORD}@db:{RDS_PORT}/{RDS_DB_NAME}?sslmode=disable"
    print("Connection string:", connection_string)
    conn = psycopg2.connect(connection_string)
    conn.autocommit = True
    url = make_url(connection_string)
    vector_store = PGVectorStore.from_params(
        database=RDS_DB_NAME,
        host=url.host,
        password=url.password,
        port=url.port,
        user=url.username,
        table_name=table_name,
        embed_dim=1536,  # openai embedding dimension
    )
    return vector_store