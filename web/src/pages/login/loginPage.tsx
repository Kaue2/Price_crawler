import { AppTitle, SwitchSigns, InputForm, Form, BtnForm } from "../../components/login";
import { useState } from "react";

function Login() {
  const [activeTab, setActiveTab] = useState<"signIn" | "signUp">("signIn");
  const [userEmail, setUserEmail] = useState("");
  const [userPassword, setUserPassword] = useState("");

  function ConfirmLogin() {
    if (userEmail == "") return;
    if (userPassword == "") return;

    console.log(userEmail);
    console.log(userPassword);
  }


  return(
    <div className="flex flex-col items-center gap-12 py-6">
      <AppTitle />
      <SwitchSigns option={activeTab} setOption={setActiveTab}/>
      <Form>
        <h2 className="text-3xl">Bem vindo de volta!</h2>
        <InputForm text="exemplo@gmail.com" type="text" action={setUserEmail} />
        <InputForm text="password" type="password" action={setUserPassword} />
        <BtnForm text="Sign In" action={ConfirmLogin} />
      </Form>
    </div>
  )
}

export default Login