
export function AppTitle() {
  return(
    <>
      <h1 className="text-white text-5xl">Price Crawler</h1>
    </>
  )
}

interface SwitchSignsProps {
  option: "signIn" | "signUp";
  setOption: (val: "signIn" | "signUp") => void; // função recebida do pai
}

export function SwitchSigns({option, setOption}: SwitchSignsProps) {
  return(
    <div className="flex gap-2">
      <button onClick={() => setOption("signIn")}>
        <h3 className={`${option == "signIn" ? "text-white" : "text-[#545454]"}`}>
          Sign In
        </h3>
      </button>
      <h3 className="text-white">|</h3>
      <button onClick={() => setOption("signUp")}>
        <h3 className={`${option == "signUp" ? "text-white" : "text-[#545454]"}`}>
          Sign Up
        </h3>
      </button>
    </div>
  )
}