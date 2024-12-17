import React, { useState } from 'react';
import './App.css';

function App() {
  const [noteText, setNoteText] = useState<string>("");
  const [token, setToken] = useState<string>("");
  const [username, setUsername] = useState<string>("");

  const onNoteInputFieldChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    const value = event.target.value;
    setNoteText(value);
  }

  const onTokenInputFieldChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    const value = event.target.value;
    setToken(value);
  }

  const onNoteButtonClicked = () => {
    fetch("http://localhost:3030", {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({
        text: noteText,
        token: token
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
        request_type: "username",
        token: token
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
      <h2>settings</h2>
      <div>
        <p>token</p>
        <input
          type="text"
          value={token}
          onChange={onTokenInputFieldChange}
          placeholder="Your access token"
        />
      </div>
      <h2>control</h2>
      <div>
        <p>{username}</p>
        <div>
          <input
            type="text"
            value={noteText}
            onChange={onNoteInputFieldChange}
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
    </div>
  );
}

export default App;
