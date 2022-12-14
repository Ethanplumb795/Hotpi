from flask import Flask
from config import Config
from flask_bootstrap import Bootstrap

app = Flask(__name__)
app.config.from_object(Config)

from hello_app import routes
bootstrap = Bootstrap(app)
