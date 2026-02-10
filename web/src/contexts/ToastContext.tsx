import { createContext, useContext, useState, type ReactNode } from 'react';

type ToastType = 'success' | 'error';

interface ToastContextData {
  showToast: (message: string, type?: ToastType) => void;
}

const ToastContext = createContext<ToastContextData>({} as ToastContextData);

function ToastMessage({ message, type, isVisible }: 
  {message: string, type: ToastType, isVisible: boolean}) {
  const styles = {
    success: "bg-green-500 text-white",
    error: "bg-red-500 text-white",
  }

  return(
    <div className={`fixed top-4 left-1/2 transform -translate-x-1/2 z-50 flex
      items-center gap-3 px-6 py-3 rounded-md shadow-2xl transition-all 
      duration-500 ease-in-out ${
        isVisible ? "translate-y-0 opacity-100" : "translate-y-20 opacity-0"
      } ${styles[type]}`}>
        <span className="font-medium text-sm">{message}</span>
    </div>
  )
}

export function ToastProvider({ children }: { children: ReactNode }) {
  const [message, setMessage] = useState("");
  const [type, setType] = useState<ToastType>('error');
  const [isVisible, setIsVisible] = useState(false);

  const showToast = (msg: string, type: ToastType = 'error') => {
    setMessage(msg);
    setType(type);
    setIsVisible(true);

    setTimeout(() => {
      setIsVisible(false);
    }, 3000);
  };

  return(
    <ToastContext.Provider value={{ showToast }}>
      {children}
      <ToastMessage message={message} type={type} isVisible={isVisible} />
    </ToastContext.Provider>
  )
}

// eslint-disable-next-line react-refresh/only-export-components
export function useToast() {
  const context = useContext(ToastContext);
  if (!context) {
    throw new Error('useToast deve ser usado dentro de um ToastProvider');
  }
  return context;
}