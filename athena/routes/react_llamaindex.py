from flask import render_template, request
from llama_index.indices.vector_store import VectorStoreIndex
from llama_index.vector_stores import PGVectorStore
import psycopg2
from sqlalchemy import make_url
from llama_index.agent import ReActAgent
from llama_index.tools import QueryEngineTool, ToolMetadata

table_name = "llmama_rag_vector"

def react_llamaindex_routes(app, connection_string):

    query_engine = llamaindex_query_engine(connection_string)

    @app.route("/react")
    def home_llamaindex():
        return render_template("index.html", agent="ReActAgent", get_endpoint="/react_get")

    @app.route("/react_get")
    def get_llamaindex_bot_response():
        userText = request.args.get('msg')
        response = query_engine.chat(userText)
        return str(response)


def llamaindex_query_engine(connection_string):

    vector_store = get_vector_store(connection_string, table_name)
    index = VectorStoreIndex.from_vector_store(vector_store)
    query_engine = index.as_query_engine(similarity_top_k=5)

    query_engine_tools = [
        QueryEngineTool(
            query_engine=query_engine,
            metadata=ToolMetadata(
                name="pal_graham_essay",
                description=(
                    "Essay of Paul Graham about how to get startup ideas"
                    "Use detailed questions abotu Paul Graham as an input"
                ),
            ),
        ),
    ]

    agent = ReActAgent.from_tools(query_engine_tools, verbose=True)
    
    return agent


def get_vector_store(connection_string, table_name):
    conn = psycopg2.connect(connection_string)
    conn.autocommit = True
    url = make_url(connection_string)
    vector_store = PGVectorStore.from_params(
        database=url.database,
        host=url.host,
        password=url.password,
        port=url.port,
        user=url.username,
        table_name=table_name,
        embed_dim=1536,  # openai embedding dimension for text-embedding-ada-002
    )
    return vector_store
