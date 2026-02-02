import pika
import os
import sys
import json
import subprocess
from dotenv import load_dotenv

def process_message(ch, method, properties, body):
  try:
    payload = json.loads(body)
    target_url = payload.get('url')

    if not target_url:
      print("ERROR: mensagem inválida")
      ch.basic_ack(delivery_tag=method.delivery_tag)

    print(f"Iniciando scrapy em: {target_url}")

    result = subprocess.run(
      [
        "scrapy", "crawl", "kabum",
        "-a", f"target_url={target_url}"
      ],
      capture_output=True,
      text=True
    )

    if result.returncode == 0:
       print("Sucesso!")
       ch.basic_ack(delivery_tag=method.delivery_tag)
    else:
       print("Falha!")
       ch.basic_ack(delivery_tag=method.delivery_tag)

  except json.JSONDecodeError:
        print("ERROR: O payload não é um JSON válido.")
        ch.basic_ack(delivery_tag=method.delivery_tag)
  except Exception as e:
        print(f"Erro crítico no consumidor: {e}")

def main():
  # carregando o .env correto que está na pasta pai da atual
  current_dir = os.path.dirname(os.path.abspath(__file__))
  env_path = os.path.join(current_dir, '..', '.env')
  load_dotenv(env_path)

  mq_user = os.getenv("MQ_USER")
  mq_password = os.getenv("MQ_PASS")
  print(f"User: {mq_user} Pass {mq_password}")
  credentials = pika.PlainCredentials(mq_user, mq_password)
  rabbit_conn = pika.BlockingConnection(
    pika.ConnectionParameters(host="localhost", credentials=credentials)
  )
  channel = rabbit_conn.channel()
  channel.basic_qos(prefetch_count=1)
  channel.basic_consume(queue="scrapping_tasks", on_message_callback=process_message)
  print('Aguardando URLs na fila')

  try:
    channel.start_consuming()
  except KeyboardInterrupt:
    print("Saindo do programa")
    sys.exit(0)

if __name__ == '__main__':
  main()