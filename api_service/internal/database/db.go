package database

import (
	"database/sql"
	"fmt"

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