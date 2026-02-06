package database

import (
	"context"
	"database/sql"
	"fmt"
	"price-crawler-api/internal/models"
	"time"

	_ "github.com/jackc/pgx/v5/stdlib"
)

type Store struct {
	db *sql.DB
}

func NewStore(connString string) (*Store, error) {
	db, err := sql.Open("pgx", connString)

	if err != nil {
		return nil, fmt.Errorf("ERROR: falha ao abrir conexão com Postgre: %w", err)
	}

	if err := db.Ping(); err != nil {
		return nil, fmt.Errorf("ERROR: falha no handshake com Postgre: %w", err)
	}

	s := &Store{db: db}

	return s, nil
}

func (s *Store) Close() {
	s.db.Close()
}

func (s *Store) GetAll() ([]models.Product, error) {
	ctx, cancel := context.WithTimeout(context.Background(), 5 * time.Second)
	defer cancel()
	
	query := `
					SELECT * FROM products
	`
	rows, err := s.db.QueryContext(ctx, query)
	if err != nil {
		return nil, fmt.Errorf("ERROR: falha ao buscar pelos produtos: %w", err)
	}
	defer rows.Close()

	var products []models.Product
	for rows.Next() {
		var prod models.Product
		if err := rows.Scan(
			&prod.Id, &prod.Url, &prod.Title, &prod.Store, 
			&prod.Last_checked_at, &prod.Created_at); err!= nil {
				return products, err
			}
			products = append(products, prod)
	}

	if err = rows.Err(); err != nil {
		return products, err
	}

	return products, nil
}