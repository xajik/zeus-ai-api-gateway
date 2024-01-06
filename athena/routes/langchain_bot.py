from flask import render_template, request
from langchain.chains import LLMChain
from langchain.memory import ConversationBufferMemory
from langchain.prompts import (
    ChatPromptTemplate,
    HumanMessagePromptTemplate,
    MessagesPlaceholder,
    SystemMessagePromptTemplate,
)
from langchain_community.chat_models import ChatOpenAI

model_4_t = "gpt-4-1106-preview"


def langchain_routes(app):

    conversation = init_openai()

    @app.route("/langchain")
    def home_langchain():
        return render_template("index.html", agent="Langchain", get_endpoint="/get_langchain")

    @app.route("/get_langchain")
    def get_langchain_bot_response():
        userText = request.args.get('msg')
        response = conversation.run({"question": userText})
        return response


def init_openai():
    llm = ChatOpenAI(model_name=model_4_t)
    prompt = ChatPromptTemplate(
        messages=[
            SystemMessagePromptTemplate.from_template(
                "You are a helpful personal assistant."
            ),
            MessagesPlaceholder(variable_name="chat_history"),
            HumanMessagePromptTemplate.from_template("{question}"),
        ]
    )
    memory = ConversationBufferMemory(
        memory_key="chat_history", return_messages=True)
    conversation = LLMChain(llm=llm, prompt=prompt,
                            verbose=True, memory=memory)
    return conversation
