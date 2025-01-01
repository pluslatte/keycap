import React, { useEffect, useState } from 'react';
import './App.css';
import NoteElement, { Note } from "./NoteElement";

function App() {
  const targetBackendAddress = "/";

  const [noteText, setNoteText] = useState<string>("");
  const [serverDomain, setServerDomain] = useState<string>("");
  const [token, setToken] = useState<string>("");
  const [username, setUsername] = useState<string>("");
  const [notes, setNotes] = useState<Note[]>();
  const [timelineType, setTimelineType] = useState<string>("");
  const [intervalTimer, setIntervalTimer] = useState<NodeJS.Timer>();
  const [isAutoReload, setIsAutoReload] = useState<boolean>(false);

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
    fetch(targetBackendAddress, {
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
    setNoteText("");
  };

  const onGetUserNameButtonClicked = async () => {
    await getUsersUserName();
  };

  const getTimeline = async (request_type_str: string) => {
    const response = await fetch(targetBackendAddress, {
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
    setTimelineType("HOME");
  };

  const onGetLocalTimelineClicked = async () => {
    await getTimeline("timelineLocal")
    setTimelineType("LOCAL");
  };

  const onGetSocialTimelineClicked = async () => {
    await getTimeline("timelineSocial")
    setTimelineType("SOCIAL");
  };

  const onGetGlobalTimelineClicked = async () => {
    await getTimeline("timelineGlobal")
    setTimelineType("GLOBAL");
  };

  const getUsersUserName = async () => {
    const response = await fetch(targetBackendAddress, {
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

  useEffect(() => {
    if (intervalTimer != null) clearInterval(intervalTimer);
    if (!isAutoReload) return;
    if (timelineType !== "") {
      setIntervalTimer(setInterval(() => {
        if (timelineType === "HOME") {
          onGetHomeTimelineClicked();
        } else if (timelineType === "LOCAL") {
          onGetLocalTimelineClicked();
        } else if (timelineType === "SOCIAL") {
          onGetSocialTimelineClicked();
        } else if (timelineType === "GLOBAL") {
          onGetGlobalTimelineClicked();
        }
      }, 5000));
    }
  }, [timelineType, isAutoReload]);

  return (
    <div className="App">
      <h1>keycap</h1>
      <a href="https://github.com/pluslatte/keycap">GitHub Repository</a>
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
          name="username"
          value={serverDomain}
          onChange={onServerDomainInputFieldChange}
          placeholder="Misskey server domain"
        />
        <p>token</p>
        <input
          type="password"
          name="password"
          value={token}
          onChange={onTokenInputFieldChange}
          placeholder="Your access token"
        />
      </div>
      <h2>timeline: {timelineType}</h2>
      <label>
        <input type="checkbox" name="autoReload" onChange={(event) => {
          setIsAutoReload(event.target.checked);
        }} />{"Auto reload (5s)"}
      </label>
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
