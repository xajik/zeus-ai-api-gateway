from flask import request, jsonify
from transformers import pipeline

def initialize_routes(app):
# Initialize the model
    model_name = "baai/bge-base-en-v1.5"
    embedder = pipeline('feature-extraction', model=model_name)

    @app.route('/embed', methods=['POST'])
    def embed():
        data = request.json
        text = data.get("text")
        print("Calling embed with text: {}", text)
        embedding = embedder(text)
        return jsonify(embedding)
