import { Component } from "@angular/core";
import { Cmd } from "src/app/interfaces/cmd";
import Note from "src/app/interfaces/note";
import invokeFactory from "src/app/utils/invoke_pipe";

@Component({
  selector: "app-note-add",
  templateUrl: "./note-add.component.html",
  styles: [],
})
export class NoteAddComponent {
  addNote(event: Note): void {
    console.log(event);
    console.log(event.title);
    console.log(event.body);
    invokeFactory(Cmd.ADD_NOTE, {
      title: event.title,
      body: event.body,
    }).subscribe();
  }
}
