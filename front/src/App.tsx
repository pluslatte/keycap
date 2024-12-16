import React from 'react';
import './App.css';

const onNoteButtonClicked = () => {
  fetch("http://localhost:3030", {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify({
      text: "テスト"
    }),
  }).then((response) => {
    if (!response.ok) {
      console.error(response);
      throw new Error("status is not 200");
    }
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
