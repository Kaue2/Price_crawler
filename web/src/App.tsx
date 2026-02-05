import { Navigate, Route, Routes } from 'react-router-dom'
import Login from './pages/login/loginPage'
import Produtos from './components/produtos'
import Wishlist from './components/wishlist'

import './App.css'
import ProductDetails from './components/produto_detalhe'

function App() {
  return (
    <>
        <Routes>
          <Route path="/login" element={<Login />} />
          

          <Route path="/app">
            <Route index element={<Navigate to="produtos" />} />
            <Route path="produtos" element={<Produtos />} />
            <Route path="wishlist" element={<Wishlist />} />
            <Route path="produto/:id" element={<ProductDetails />} />
          </Route>

          <Route path='*' element={<Navigate to="/login" />} />
        </Routes>
    </>
  )
}

export default App
