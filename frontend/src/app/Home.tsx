import { useNavigate } from '@tanstack/react-router'

function Home() {
  const navigate = useNavigate()
  const username = localStorage.getItem('username') ?? 'there'

  function handleLogout() {
    localStorage.removeItem('token')
    localStorage.removeItem('username')
    navigate({ to: '/login' })
  }

  return (
    <div className="min-h-screen flex flex-col items-center justify-center bg-gray-50">
      <div className="w-full max-w-md bg-white rounded-2xl shadow-md p-8 text-center">
        <h1 className="text-3xl font-bold text-gray-900 mb-2">Welcome, {username}!</h1>
        <p className="text-gray-500 mb-8">You're logged in to Calculi.</p>
        <button
          onClick={handleLogout}
          className="bg-red-500 hover:bg-red-600 text-white font-medium rounded-lg px-6 py-2 text-sm transition-colors"
        >
          Log out
        </button>
      </div>
    </div>
  )
}

export default Home
