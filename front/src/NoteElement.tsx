import { useState } from "react";

export type User = {
    id: string,
    createdAt: string,
    username: string,
    host: string | null,
    name: string,
    onlineStatus: string,
    avatarUrl: string,
    avatarBlurhash: string,
}

export type Note = {
    id: string,
    createdAt: string,
    text: string | null,
    cw: string | null,
    user: User,
    userId: string,
    visibility: string,
}


export default function NoteElement(note: Note) {

    return (
        <div className="Note">
            <h3>{note?.user.username}{note?.user.host ? "@" + note?.user.host : ""}</h3>
            {note?.text ? <p>{note?.text}</p> : null}
        </div>
    );
}