package services

import (
	"database/sql"
	"errors"
	"price-crawler-api/internal/services/database"
	"regexp"

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

type UserServiceRequestBody struct {
	Email string `json:"email"`
	PasswordPlain string `json:"password-plain"`
}

func validateEmail(e string) bool{
	return  emailRegex.MatchString(e)
}

func (s *UserService) Create(email string, plainPassword string) error {
	valid := validateEmail(email)
	if valid != true {
		return ErrInvalidEmail
	}

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