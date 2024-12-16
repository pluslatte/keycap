import React from 'react';
import './App.css';

const onNoteButtonClicked = () => {
  fetch("http://localhost:3030", {
    method: "POST",
    body: JSON.stringify({
      text: "テスト"
    }),
  }).then((response) => {
    if (!response.ok)
      throw new Error("status is not 200");
  });
};

function App() {
  return (
    <div className="App">
      <button onClick={onNoteButtonClicked}>
        ノート
      </button>
    </div>
  );
}

export default App;
