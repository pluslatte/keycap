import React, { useState } from 'react';
import './App.css';
import NoteElement, { Note } from "./NoteElement";

function App() {
  const [noteText, setNoteText] = useState<string>("");
  const [serverDomain, setServerDomain] = useState<string>("");
  const [token, setToken] = useState<string>("");
  const [username, setUsername] = useState<string>("");
  const [notes, setNotes] = useState<Note[]>();

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

  const getTimeline = async (request_type_str: string) => {
    const response = await fetch("http://localhost:3030", {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({
        request_type: request_type_str,
        server_domain: serverDomain,
        token: token
      }),
    })

    if (!response.ok) {
      console.error(response);
      return;
    }

    const notes: Note[] = await response.json();
    setNotes(notes);
  };

  const onGetHomeTimelineClicked = async () => {
    await getTimeline("timelineHome")
  };

  const onGetLocalTimelineClicked = async () => {
    await getTimeline("timelineLocal")
  };

  const onGetSocialTimelineClicked = async () => {
    await getTimeline("timelineSocial")
  };

  const onGetGlobalTimelineClicked = async () => {
    await getTimeline("timelineGlobal")
  };

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
        <button onClick={onGetHomeTimelineClicked}>get HOME timeline</button>
        <button onClick={onGetLocalTimelineClicked}>get LOCAL timeline</button>
        <button onClick={onGetSocialTimelineClicked}>get SOCIAL timeline</button>
        <button onClick={onGetGlobalTimelineClicked}>get GLOBAL timeline</button>
        <p />
        {notes?.map((note) => NoteElement(note))}
      </div>
    </div>
  );
}

export default App;
