import { useEffect, useState } from "react";

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

interface InputOptions{
  text: string;
  type: "text" | "password";
  action: (val: string) => void;
}

export function InputForm({ text, type, action }: InputOptions) {
  const [inputValue, setInputValue] = useState("");

  useEffect(() => {
    action(inputValue)
  }, [inputValue, action]);

  return(
    <>
      <input 
        className="border border-black px-3 py-4 rounded-md w-full" 
        type={type} placeholder={text}
        value={inputValue}
        onChange={(e) => setInputValue(e.target.value)}
      />
    </>
  )
}

interface FormProps {
  children: React.ReactNode
}

export function Form({ children }: FormProps){
  return(
    <div className="flex flex-col items-center gap-8 rounded-md
    bg-[#F8F8F8] px-5 py-12 ">
      {children}      
    </div>
  )
}

interface BtnFormProps{
  text: string;
  action: () => void;
}

export function BtnForm({ text, action }: BtnFormProps){
  return(
    <>
      <button onClick={() =>  action()}
        className="bg-black text-white rounded-md
        py-4 w-full text-center">{text}</button>
    </>
  )
}