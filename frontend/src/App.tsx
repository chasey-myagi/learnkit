import { BrowserRouter, Routes, Route } from 'react-router-dom';
import { AppHeader } from './components/AppHeader';
import { ProgramList } from './pages/ProgramList';
import { ProgramDetail } from './pages/ProgramDetail';

function App() {
  return (
    <BrowserRouter>
      <AppHeader />
      <Routes>
        <Route path="/" element={<ProgramList />} />
        <Route path="/program/:slug" element={<ProgramDetail />} />
      </Routes>
    </BrowserRouter>
  );
}

export default App;
