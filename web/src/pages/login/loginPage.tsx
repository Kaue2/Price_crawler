import { AppTitle, SwitchSigns } from "../../components/login";
import { useState } from "react";

function Login() {
  const [activeTab, setActiveTab] = useState<"signIn" | "signUp">("signIn");

  return(
    <div className="flex flex-col items-center gap-12">
      <AppTitle />
      <SwitchSigns option={activeTab} setOption={setActiveTab}/>
    </div>
  )
}

export default Login