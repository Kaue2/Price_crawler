import pymongo
import json
import os 
import pika
from dotenv import load_dotenv

load_dotenv()

class MongoRabbitPipeline(): 
    def __init__(self):
        mongo_user = os.getenv("MONGO_USER")
        mongo_pass = os.getenv("MONGO_PASSWORD")
        print("MONGO USER: " + mongo_user)
        print("MONGO PASSWORD: " + mongo_pass)
        mongo_uri = f"mongodb://{mongo_user}:{mongo_pass}@localhost:27017"

        self.mongo_client = pymongo.MongoClient(mongo_uri)
        self.db = self.mongo_client["crawler_db"]
        self.collection = self.db["raw_pages"]

        mq_user = os.getenv("MQ_USER")
        mq_password = os.getenv("MQ_PASS")

        credentials = pika.PlainCredentials(mq_user, mq_password)
        self.rabbit_connection = pika.BlockingConnection(
            pika.ConnectionParameters(host="localhost", credentials=credentials)
        )
        self.channel = self.rabbit_connection.channel()

        self.channel.queue_declare(queue='raw_prices', durable=True)

    def process_item(self, item, spider):
        data = dict(item)

        try:
            self.collection.insert_one(data)
            spider.log(f"Html salvo: {data['_id']}")
        except Exception as e: 
            spider.log(f"Erro ao salvar no mongo: {e}")
            return item
        
        message = {
            "id": data["_id"], 
            "url": data["url"],
        }
        

        self.channel.basic_publish(
            exchange='',
            routing_key='raw_prices',
            body=json.dumps(message),
            properties=pika.BasicProperties(
                delivery_mode=2
            )
        )
        spider.log(f"Mensagem enviada: {message['id']}")

        return item
    
    def close_spider(self, spider):
        self.mongo_client.close()
        self.rabbit_connection.close()
        