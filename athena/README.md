# Athena 

Athena is a Python-based server designed to use Hugging Face `transformers` library, Langchain, Llamaindex OpenAi and other cool APIs.

## Prerequisites

Before running the Athena server, ensure you have the following installed:

- Python 3.9 or higher
- pip


## Running the Server

1. ```python app..py```

###

Chat with different agents at:: 
* LLangchain `http://127.0.0.1:3005/langchain`
* LLamaindex: `http://127.0.0.1:3005/llamaindex`
* OpenAI: `http://127.0.0.1:3005/openai`
* Llamaindex Postgress NLSQL: `http://127.0.0.1:3005/nlsql` - talk with main DB

## Docker 

If you prefer to run Athena in a Docker container:

1. **Build the Docker Image** ```docker build -t athena-server .```

2. **Run the Container** ```docker run -p 3005:3005 athena-server```

### Memory

Runnin model might require a lot of memory, in case Docker fail with `133` add RAM to the conainer: 
    
* ```colima start --memory 8```


