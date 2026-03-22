import { BrowserRouter, Routes, Route } from 'react-router-dom';
import { ErrorBoundary } from './components/ErrorBoundary';
import { AppHeader } from './components/AppHeader';
import { ProgramList } from './pages/ProgramList';
import { ProgramDetail } from './pages/ProgramDetail';

function App() {
  return (
    <ErrorBoundary>
      <BrowserRouter>
        <a href="#main-content" className="skip-to-content">
          跳转到主要内容
        </a>
        <AppHeader />
        <div id="main-content">
          <Routes>
            <Route path="/" element={<ProgramList />} />
            <Route path="/program/:slug" element={<ProgramDetail />} />
          </Routes>
        </div>
      </BrowserRouter>
    </ErrorBoundary>
  );
}

export default App;
