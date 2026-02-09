package services

import (
	"database/sql"
	"price-crawler-api/internal/services/database"
	"github.com/google/uuid"
	"golang.org/x/crypto/bcrypt"
)

type UserService struct {
	db *sql.DB
}

func NewUserService(db *database.Store) *UserService {
	return &UserService{
		db: db.Db,
	}
}

type UserServiceRequestBody struct {
	Email string `json:"email"`
	PasswordPlain string `json:"password"`
}

func (s *UserService) Create(email string, plainPassword string) error {
	hashedPassword, err := bcrypt.GenerateFromPassword([]byte(plainPassword), bcrypt.DefaultCost)
	if err != nil {
		return err
	}
	
	password_hash := string(hashedPassword)
	id := uuid.New()

	query := `
		INSERT INTO users (id, email, password_hash)
		VALUES ($1, $2, $3)
	`
	_, err = s.db.Exec(query, id, email, password_hash)

	return err
} 