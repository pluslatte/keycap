import { Note } from "./NoteElement";

export default function ReNoteElement(note: Note) {

    return (
        <div className="Note">
            <h4>{`Renote -> ${note?.user.name} : ${note?.user.username}`}{note?.user.host ? "@" + note?.user.host : ""}</h4>
            {note?.text ? <p>{note?.text}</p> : null}
        </div>
    );
}