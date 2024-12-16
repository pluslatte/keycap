import React, { useState } from 'react';
import './App.css';

function App() {
  const [inputValue, setInputValue] = useState<string>("");

  const onInputFieldChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    const value = event.target.value;
    setInputValue(value);
  }

  const onNoteButtonClicked = () => {
    fetch("http://localhost:3030", {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({
        text: inputValue
      }),
    }).then((response) => {
      if (!response.ok) {
        console.error(response);
        throw new Error("status is not 200");
      }
    });
  };

  return (
    <div className="App">
      <div>
        <input
          type="text"
          value={inputValue}
          onChange={onInputFieldChange}
          placeholder="Type something..."
        />
      </div>
      <button onClick={onNoteButtonClicked}>
        ノート
      </button>
    </div>
  );
}

export default App;
