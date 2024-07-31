import logo from './logo.svg';
import axios from 'axios';
import {React, useState} from 'react'
import { BrowserRouter as Router, Routes, Route } from "react-router-dom";
import Chessboard from './Chessboard';


function App() {

  // let ret = axios.get("http://localhost:3001/board");
  // let [requestVal, setRequestVal] = useState("")
  // ret.then(function (response) {
  //   setRequestVal(response.data);
  // })

  return (
    <div className="bg-black">
      <Router>
        <Routes>
          <Route path="/" element={<Chessboard />} />
        </Routes>
      </Router>
    </div>
  );
}

export default App;
