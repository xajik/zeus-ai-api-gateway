from flask import render_template, request
from openai import OpenAI
import os

model_4_t = "gpt-4-1106-preview"

def get_completion(client, prompt, model=model_4_t):
    messages = [{"role": "user", "content": prompt}]
    response = client.chat.completions.create(
        model=model, messages=messages, temperature=0)
    return response.choices[0].message.content


def chat_bot_routes(app):

    client = init_openai()

    @app.route("/openai")
    def home_openai():
        return render_template("index.html", agent="OpenAI",get_endpoint="/get_openai")

    @app.route("/get_openai")
    def get_openai_bot_response():
        userText = request.args.get('msg')
        response = get_completion(client, userText)
        return response


def init_openai():
    OPENAI_API_KEY = os.environ.get("OPENAI_API_KEY")
    return OpenAI()
