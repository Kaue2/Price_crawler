import { AppTitle, SwitchSigns, InputForm, Form, BtnForm } from "../../components/login";
import React, { useState } from "react";
import { authService } from "../../services/authService";
import { useToast } from "../../contexts/ToastContext";

function Login() {
  const [activeTab, setActiveTab] = useState<"signIn" | "signUp">("signIn");
  const [userEmail, setUserEmail] = useState("");
  const [userPassword, setUserPassword] = useState("");
  const [userName, setUserName] = useState("");
  const { showToast } = useToast();

  const isValidEmail = (email: string) => {
    return /\S+@\S+\.\S+/.test(email);
  }

  const handleClick = async (e: React.MouseEvent) => {
    e.preventDefault();

    if (!isValidEmail(userEmail)) {
      showToast("Erro: Email inválido", "error")
      return;
    }

    if (activeTab === "signIn") login();
    else registrar();
  }

  const registrar = async() => {
    try {
      await authService.register({
        username: userName,
        email: userEmail,
        password_plain: userPassword
      });
    } catch (err) {
      showToast("Erro: falha ao criar usuário", "error")
      console.error(err)
    } finally {
      showToast("Usuário criado com sucesso!", "success");
      console.log("Usuário criado");
    }
  }

  const login = async() => {
    try {
      await authService.signIn({
        email: userEmail,
        password_plain: userPassword
      });
    } catch (err) {
      console.log(err);
    } finally {
      showToast("Usuário Autenticado com Sucesso!", "success");
      console.log("Usuário autenticado com sucesso!");
    }
  }


  return(
    <div className="flex flex-col items-center gap-12 py-6">
      <AppTitle />
      <SwitchSigns option={activeTab} setOption={setActiveTab}/>
      <Form key={activeTab} animation="animate-fade-in">
        <h2 className="text-3xl">
          {activeTab === "signIn" ? "Bem vindo de volta!" : "Realize seu cadastro!"}
        </h2>
        {activeTab === "signUp" && (
          <div className="w-full">
            <InputForm text="user name" type="text" action={setUserName} />
          </div>
        )}
        <InputForm text="exemplo@gmail.com" type="email" action={setUserEmail} />
        <InputForm text="password" type="password" action={setUserPassword} />
        <BtnForm text={activeTab === "signIn" ? "Sign In" : "Sign Up"} action={handleClick} />
      </Form>
    </div>
  )
}

export default Login