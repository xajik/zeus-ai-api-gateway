from flask import Flask
from routes.embedding import initialize_routes

app = Flask(__name__)

initialize_routes(app)

if __name__ == '__main__':
    print("Starting Athena...")
    app.run(host='0.0.0.0', debug=True, port=3005)
