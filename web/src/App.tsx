import { Link, Route, Routes } from 'react-router'
import Login  from './components/login'

import './App.css'

function App() {
  return (
    <>
      <h1 className="text-white">Price Crawler</h1>

      <Navigation />
      <Routes>
        <Route >
          <Route index element={<Login />} />
          <Route path='users' element />  
        </Route>
      </Routes>
    </>
  )
}

function Navigation() {
  return (
    <nav className="border border-solid pb-4">
      <Link to="/login">login</Link>
      <Link to="/users">Users</Link>
    </nav>
  )
}

export default App
