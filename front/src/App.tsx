import React, { useState } from 'react';
import './App.css';

function App() {
  const [inputValue, setInputValue] = useState<string>("");
  const [username, setUsername] = useState<string>("");

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

  const onGetUserNameButtonClicked = async () => {
    await getUsersUserName();
  };

  const getUsersUserName = async () => {
    const response = await fetch("http://localhost:3030", {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({
        req_type: "username"
      }),
    })

    if (!response.ok) {
      console.error(response);
      return;
    }

    let username = await response.text();

    setUsername(username);
  };

  return (
    <div className="App">
      <h1>keycap</h1>
      <p>{username}</p>
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
      <button onClick={onGetUserNameButtonClicked}>
        get username
      </button>
    </div>
  );
}

export default App;
