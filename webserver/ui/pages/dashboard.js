
import React, { useEffect, useState } from 'react'
import useLocalStorage from '../libs/useLocalStorage'
import Login from '../components/login';

export default function Dashboard() {

  const [showLogIn, setShowLogIn] = useState(false);
  const [auth, setAuthUser] = useLocalStorage("user", false);

  useEffect(() => setShowLogIn(!auth), [auth])

  return (
        <Login show={showLogIn} setAuth={setAuthUser} />
  )
  }