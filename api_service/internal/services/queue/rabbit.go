package queue

import (
	"fmt"
	ampq "github.com/rabbitmq/amqp091-go"
)

type RabbitMQ struct {
	conn *ampq.Connection
	channel *ampq.Channel
}

func NewRabbitMQConnection(connString string) (*RabbitMQ, error){
	conn, err := ampq.Dial(connString)
	if err  != nil {
		return nil, fmt.Errorf("ERROR: falha ao conectar com RabbitMQ: %w", err)
	}

	ch, err := conn.Channel()
	if err != nil {
		return nil, fmt.Errorf("ERROR: falha ao abrir canal RabbitMQ: %w", err)
	}

	rabbitMQClient := &RabbitMQ{
		conn: conn,
		channel: ch,
	}

	return rabbitMQClient, nil;
}

func (r *RabbitMQ) Close() {
	r.conn.Close()
	r.channel.Close()
}