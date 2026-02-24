package services

import (
	"context"
	"database/sql"
	"errors"
	"price-crawler-api/internal/services/database"
	"regexp"
	"time"

	"github.com/google/uuid"
	"golang.org/x/crypto/bcrypt"
)

const emailRegexPattern = `^[a-zA-Z0-9._%+\-]+@[a-zA-Z0-9.\-]+\.[a-zA-Z]{2,4}$`

var emailRegex = regexp.MustCompile(emailRegexPattern)
var ErrInvalidEmail = errors.New("ERRO: email fornecido é inválido")

type UserService struct {
	db *sql.DB
}

func NewUserService(db *database.Store) *UserService {
	return &UserService{
		db: db.Db,
	}
}

type CreateUserRequestBody struct {
	UserName      string `json:"user-name"` 
	Email         string `json:"email"`
	PasswordPlain string `json:"password-plain"`
}

type LoginRequestBody struct {
	Email         string `json:"email"`
	PasswordPlain string `json:"password-plain"`
}

type LoginResponse struct {
	Email         string `json:"email"` 
}

func validateEmail(e string) bool {
	return emailRegex.MatchString(e)
}

func (s *UserService) Create(userName string, email string, plainPassword string) error {
	valid := validateEmail(email)
	if valid != true {
		return ErrInvalidEmail
	}

	hashedPassword, err := bcrypt.GenerateFromPassword([]byte(plainPassword), bcrypt.MinCost)
	if err != nil {
		return err
	}

	password_hash := string(hashedPassword)
	id := uuid.New()

	query := `
		INSERT INTO users (id, user_name, email, password_hash)
		VALUES ($1, $2, $3, $4)
	`
	_, err = s.db.Exec(query, id, userName, email, password_hash)

	return err
}

func (s *UserService) Login(email string, passwordPlain string) error {
	var response LoginResponse
	hashedPassword, err := bcrypt.GenerateFromPassword([]byte(passwordPlain), bcrypt.MinCost)

	if err != nil {
		return err
	}

	password_hash := string(hashedPassword)

	query := `
						SELECT * 
						FROM users
						WHERE email = $1
						AND password_hash = $2
	`

	ctx, cancel := context.WithTimeout(context.Background(), 5*time.Second)
	defer cancel()

	err = s.db.QueryRowContext(ctx, query, email, password_hash).Scan(&response)

	if err != nil {
		return  err
	}

	return  nil
}