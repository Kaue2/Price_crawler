interface CreateUserDTO {
  email: string;
  password_plain: string;
}

export const authService = {
  async register(data: CreateUserDTO) {
    const response = await fetch('http://localhost:8080/create-user', {
    method: 'POST',
    headers: {
      'Content-type' : 'application/json'
    },
    body: JSON.stringify(data)
    });

    if (!response.ok) {
      throw new Error('Falha ao criar usuário');
    }

    return true;
  }  
}