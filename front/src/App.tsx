import React, { useState } from 'react';
import './App.css';

function App() {
  const [noteText, setNoteText] = useState<string>("");
  const [serverDomain, setServerDomain] = useState<string>("");
  const [token, setToken] = useState<string>("");
  const [username, setUsername] = useState<string>("");

  const onNoteInputFieldChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    const value = event.target.value;
    setNoteText(value);
  };

  const onServerDomainInputFieldChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    const value = event.target.value;
    setServerDomain(value);
  };

  const onTokenInputFieldChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    const value = event.target.value;
    setToken(value);
  };

  const onNoteButtonClicked = () => {
    fetch("http://localhost:3030", {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({
        text: noteText,
        server_domain: serverDomain,
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

  const onWsButtonClicked = async () => {
    const response = await fetch("http://localhost:3030", {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({
        request_type: "ws",
        server_domain: serverDomain,
        token: token
      }),
    })

    if (!response.ok) {
      console.error(response);
      return;
    }
  }

  const getUsersUserName = async () => {
    const response = await fetch("http://localhost:3030", {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({
        request_type: "username",
        server_domain: serverDomain,
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
      <h2>control</h2>
      <div>
        <p>{username}@{serverDomain}</p>
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
        <button onClick={onWsButtonClicked}>ws</button>
      </div>
      <h2>settings</h2>
      <div>
        <p>server domain</p>
        <input
          type="text"
          value={serverDomain}
          onChange={onServerDomainInputFieldChange}
          placeholder="Misskey server domain"
        />
        <p>token</p>
        <input
          type="text"
          value={token}
          onChange={onTokenInputFieldChange}
          placeholder="Your access token"
        />
      </div>
      <h2>timeline</h2>
      <div>

      </div>
    </div>
  );
}

export default App;
