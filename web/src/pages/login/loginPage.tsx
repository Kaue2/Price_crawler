import { AppTitle, SwitchSigns, InputForm, Form, BtnForm } from "../../components/login";
import React, { useState } from "react";
import { authService } from "../../services/authService";

function Login() {
  const [activeTab, setActiveTab] = useState<"signIn" | "signUp">("signIn");
  const [userEmail, setUserEmail] = useState("");
  const [userPassword, setUserPassword] = useState("");

  const isValidEmail = (email: string) => {
    return /\S+@\S+\.\S+/.test(email);
  }

  const handleRegistar = async (e: React.MouseEvent) => {
    e.preventDefault();

    if (!isValidEmail(userEmail)) {
      return;
    }

    try {
      await authService.register({
        email: userEmail,
        password_plain: userPassword
      });
    } catch (err) {
      console.error(err)
    } finally {
      console.log("Usuário criado");
    }
  }


  return(
    <div className="flex flex-col items-center gap-12 py-6">
      <AppTitle />
      <SwitchSigns option={activeTab} setOption={setActiveTab}/>
      <Form>
        <h2 className="text-3xl">Bem vindo de volta!</h2>
        <InputForm text="exemplo@gmail.com" type="email" action={setUserEmail} />
        <InputForm text="password" type="password" action={setUserPassword} />
        <BtnForm text="Sign In" action={handleRegistar} />
      </Form>
    </div>
  )
}

export default Login